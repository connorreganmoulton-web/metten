// Dummy-only native runner: no secret arguments, environment values, files or clipboard.
import Foundation
import AppKit
import Security
import ApplicationServices
import Darwin

// Progress must survive a stalled process when stdout is redirected.
setbuf(stdout, nil)

struct Failure: Error { let message: String }
func require(_ value: Bool, _ message: String) throws { if !value { throw Failure(message: message) } }
func ok(_ status: OSStatus, _ stage: String) throws { try require(status == errSecSuccess, "\(stage) failed (OSStatus \(status))") }
func path(_ keychain: SecKeychain) throws -> String {
    var bytes = [CChar](repeating: 0, count: 4096); var length = UInt32(bytes.count)
    try ok(SecKeychainGetPath(keychain, &length, &bytes), "get owned path")
    return String(cString: bytes)
}
func metadata() throws -> [String] {
    var list: CFArray?; try ok(SecKeychainCopySearchList(&list), "search-list metadata")
    var result = try (list! as! [SecKeychain]).map { try path($0) }
    var keychain: SecKeychain?; try ok(SecKeychainCopyDefault(&keychain), "default metadata")
    result.append("default:" + (try path(keychain!))); return result
}
func attribute(_ node: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?; return AXUIElementCopyAttributeValue(node, name as CFString, &value) == .success ? value : nil
}
func nodes(_ node: AXUIElement, depth: Int = 0) -> [AXUIElement] {
    if depth > 25 { return [] }
    let children = attribute(node, kAXChildrenAttribute) as? [AXUIElement] ?? []
    return [node] + children.flatMap { nodes($0, depth: depth + 1) }
}
let fm = FileManager.default
var owned: SecKeychain?
var ownedDirectory: URL?
var failureSidecars: [(URL, UInt64)] = []
var fixture: SecKeychain?
var baselineMetadata: [String]?
var child: pid_t = 0
var closeRequested = false
var socket: Int32 = -1
var sequence = 0
var session = ""
var outputPipe: [Int32] = [-1,-1]
var collected = Data()
let a = "metten-dummy-" + UUID().uuidString
let b = "metten-dummy-" + UUID().uuidString
let password = UUID().uuidString + UUID().uuidString
func send(_ object: [String: Any]) throws {
    let data = try JSONSerialization.data(withJSONObject: object)
    var length = UInt32(data.count).bigEndian
    var packet = Data(bytes: &length, count: 4); packet.append(data)
    try packet.withUnsafeBytes { bytes in
        var offset = 0
        while offset < bytes.count {
            let n = Darwin.write(socket, bytes.baseAddress!.advanced(by: offset), bytes.count-offset)
            try require(n > 0, "private channel write failed"); offset += n
        }
    }
}
func expectation(_ marker: String, sequenceOverride: Int? = nil) throws {
    sequence += 1
    try send(["session":session,"sequence":sequenceOverride ?? sequence,"phase":"local-use","expected":marker])
    var timeout = timeval(tv_sec:3,tv_usec:0)
    setsockopt(socket,SOL_SOCKET,SO_RCVTIMEO,&timeout,socklen_t(MemoryLayout<timeval>.size))
    var ack: UInt8 = 255
    try require(read(socket,&ack,1) == 1 && ack == (sequenceOverride == nil ? 1 : 0), "expectation receipt rejected or timed out")
}
func appNodes() -> [AXUIElement] { nodes(AXUIElementCreateApplication(child)) }
func text(_ node: AXUIElement) -> String {
    [kAXTitleAttribute, kAXDescriptionAttribute, kAXValueAttribute].compactMap { attribute(node,$0) as? String }.joined(separator:" ")
}
func waitText(_ fragment: String) throws {
    let deadline = Date().addingTimeInterval(8)
    while Date() < deadline {
        if appNodes().contains(where: { text($0).contains(fragment) }) { print("PASS: " + fragment); return }
        usleep(100_000)
    }
    let statuses = ["Unavailable —", "Locked —", "Denied —", "Invalid input —", "Credential action failed", "Checking local credential action", "Already saved —", "Missing —", "Authentication failed —"]
    let observed = statuses.filter { prefix in appNodes().contains { text($0).contains(prefix) } }
    throw Failure(message: "screen did not show expected fixed status: " + fragment + "; observed categories: " + observed.joined(separator:", "))
}
func press(_ title: String) throws {
    let deadline = Date().addingTimeInterval(4)
    while Date() < deadline {
        let matches = appNodes().filter { (attribute($0,kAXRoleAttribute) as? String) == kAXButtonRole && text($0).trimmingCharacters(in:.whitespaces) == title }
        if matches.count == 1 && (attribute(matches[0], kAXEnabledAttribute) as? Bool) == true {
            try require(AXUIElementPerformAction(matches[0], kAXPressAction as CFString) == .success, "native press failed")
            return
        }
        usleep(50_000)
    }
    throw Failure(message:"native enabled button unavailable: " + title)
}
func fill(_ marker: String) throws {
    let fields = appNodes().filter { (attribute($0,kAXRoleAttribute) as? String) == kAXTextFieldRole }
    try require(fields.count == 1, "dummy input unavailable")
    try require(AXUIElementSetAttributeValue(fields[0], kAXValueAttribute as CFString, marker as CFString) == .success, "native field entry failed")
}
func check(_ marker: String, _ status: String) throws { try expectation(marker); try press("Check local credential use"); try waitText(status) }
func clearCheck() throws {
    let fields = appNodes().filter { (attribute($0,kAXRoleAttribute) as? String) == kAXTextFieldRole }
    try require(fields.count == 1 && (attribute(fields[0],kAXValueAttribute) as? String ?? "").isEmpty, "input was not cleared")
    try require(!appNodes().contains(where: { text($0).contains(a) || text($0).contains(b) }), "marker found in screen")
    print("PASS: input cleared and native screen scan")
}
func collectChildOutput() {
    if socket >= 0 { close(socket); socket = -1 }
    if outputPipe[1] >= 0 { close(outputPipe[1]); outputPipe[1] = -1 }
    if outputPipe[0] >= 0 {
        var bytes = [UInt8](repeating:0,count:8192)
        while true { let n = read(outputPipe[0],&bytes,bytes.count); if n <= 0 { break }; collected.append(contentsOf:bytes.prefix(n)) }
        close(outputPipe[0]); outputPipe[0] = -1
    }
}
func waitForChildExit(timeout: TimeInterval) throws -> Bool {
    if child == 0 { return true }
    try require(child > 0, "invalid owned child identity; state retained")
    let deadline = Date().addingTimeInterval(timeout)
    repeat {
        var status: Int32 = 0
        let result = waitpid(child,&status,WNOHANG)
        if result == child {
            child = 0 // Only a confirmed wait result permits cleanup.
            collectChildOutput()
            try require(status == 0,"owned child exited unsuccessfully")
            return true
        }
        if result < 0 && errno != EINTR { throw Failure(message:"owned child exit could not be confirmed; state retained") }
        if Date() >= deadline { return false }
        usleep(100_000)
    } while true
}
func requestQuit() throws {
    if try waitForChildExit(timeout:0) { return }
    if !closeRequested {
        let app = AXUIElementCreateApplication(child)
        let windows = attribute(app,kAXWindowsAttribute) as? [AXUIElement] ?? []
        try require(windows.count == 1, "owned window unavailable")
        guard let button = attribute(windows[0],kAXCloseButtonAttribute) else { throw Failure(message:"close button unavailable") }
        closeRequested = true // A failed AX reply does not prove the request was undelivered.
        print("STEP: request guarded window-close")
        try require(AXUIElementPerformAction(button as! AXUIElement,kAXPressAction as CFString) == .success, "window close failed")
    }
}
func quit() throws {
    try requestQuit()
    print("STEP: wait for owned app exit")
    try require(try waitForChildExit(timeout:8),"guarded quit pending; owned app retained")
    print("PASS: guarded window-close exit")
}
// No signals or repeated close requests: retain ownership until actual exit.
func awaitRetainedChildExit() -> Bool {
    var successful = true
    var reportedUnknown = false
    while child != 0 {
        do { _ = try waitForChildExit(timeout:1) }
        catch {
            successful = false
            if !reportedUnknown { print("CLEANUP: child exit failed or remains unknown; no success claimed"); reportedUnknown = true }
            if child != 0 { usleep(100_000) }
        }
    }
    return successful
}
// Process-only check: no app, Keychain, workspace or Accessibility calls.
func recoverySelfTest() throws {
    func spawn(_ executable: String) throws -> Int32 {
        var input: [Int32] = [-1,-1]
        try require(pipe(&input) == 0 && pipe(&outputPipe) == 0,"self-test pipe failed")
        var actions: posix_spawn_file_actions_t?; posix_spawn_file_actions_init(&actions)
        var attributes: posix_spawnattr_t?; posix_spawnattr_init(&attributes)
        defer { posix_spawn_file_actions_destroy(&actions); posix_spawnattr_destroy(&attributes) }
        posix_spawnattr_setflags(&attributes,Int16(POSIX_SPAWN_CLOEXEC_DEFAULT))
        posix_spawn_file_actions_adddup2(&actions,input[0],0)
        posix_spawn_file_actions_adddup2(&actions,outputPipe[1],1)
        posix_spawn_file_actions_adddup2(&actions,outputPipe[1],2)
        let args = [strdup(executable),nil]; defer { args.forEach { free($0) } }
        try ok(posix_spawn(&child,executable,&actions,&attributes,args,nil),"spawn process-only self-test")
        close(input[0])
        return input[1]
    }
    let input = try spawn("/bin/cat")
    try require(try !waitForChildExit(timeout:0.02) && child > 0,"self-test cleaned a pending child")
    print("SELF-TEST: pending child retained; waiting for test gate")
    var gate: UInt8 = 0
    try require(read(STDIN_FILENO,&gate,1) == 1,"self-test gate missing")
    close(input)
    try require(awaitRetainedChildExit() && child == 0,"self-test late exit was not recovered")
    try require(outputPipe == [-1,-1] && socket == -1,"self-test channels not closed")
    try require(try waitForChildExit(timeout:0),"self-test repeated wait failed")
    let failedInput = try spawn("/usr/bin/false"); close(failedInput)
    try require(!awaitRetainedChildExit() && child == 0,"self-test unsuccessful exit claimed success")
    print("PASS: late child exit recovered, channels closed, unsuccessful exit stays failed")
}
func launch(_ bundle: URL, _ directory: URL, _ target: SecKeychain, isolatedSetup: Bool = true) throws {
    let info = try PropertyListSerialization.propertyList(from: Data(contentsOf:bundle.appendingPathComponent("Contents/Info.plist")),format:nil) as! [String:Any]
    let executable = bundle.appendingPathComponent("Contents/MacOS/" + (info["CFBundleExecutable"] as! String)).path
    var pair: [Int32] = [-1,-1]; try require(socketpair(AF_UNIX,SOCK_STREAM,0,&pair) == 0,"socketpair failed")
    try require(pipe(&outputPipe) == 0,"output pipe failed")
    var actions: posix_spawn_file_actions_t?; posix_spawn_file_actions_init(&actions)
    var attributes: posix_spawnattr_t?; posix_spawnattr_init(&attributes)
    posix_spawnattr_setflags(&attributes,Int16(POSIX_SPAWN_CLOEXEC_DEFAULT))
    defer { posix_spawn_file_actions_destroy(&actions); posix_spawnattr_destroy(&attributes) }
    posix_spawn_file_actions_adddup2(&actions,pair[1],3)
    posix_spawn_file_actions_adddup2(&actions,outputPipe[1],1)
    posix_spawn_file_actions_adddup2(&actions,outputPipe[1],2)
    if pair[0] != 3 { posix_spawn_file_actions_addclose(&actions,pair[0]) }
    if pair[1] != 3 { posix_spawn_file_actions_addclose(&actions,pair[1]) }
    let args = [strdup(executable),nil]
    var environment = ProcessInfo.processInfo.environment
    if isolatedSetup { environment["METTEN_VERIFY_FD"] = "3" } else { environment.removeValue(forKey:"METTEN_VERIFY_FD") }
    environment["METTEN_TEST_WORKSPACE"] = directory.appendingPathComponent("workspace").path
    let env = environment.map { strdup($0.key + "=" + $0.value) } + [nil]
    defer { args.forEach { free($0) }; env.forEach { free($0) } }
    try ok(posix_spawn(&child,executable,&actions,&attributes,args,env),"spawn finished bundle")
    close(pair[1]); socket = pair[0]
    closeRequested = false
    session = UUID().uuidString.replacingOccurrences(of:"-",with:"").lowercased(); sequence = 0
    let targetPath = try path(target)
    let attrs = try fm.attributesOfItem(atPath:targetPath)
    if isolatedSetup { try send(["path":targetPath,"device":attrs[.systemNumber]!,"inode":attrs[.systemFileNumber]!,"password":password,"session":session]) }
    try waitText("Local credential readability unchecked.")
}
func scan(_ directory: URL) throws {
    let markers = [Data(a.utf8),Data(b.utf8)]
    try require(!markers.contains(where: { collected.range(of:$0) != nil }), "marker in captured output")
    for case let file as URL in fm.enumerator(at:directory,includingPropertiesForKeys:[.isRegularFileKey])! {
        if (try file.resourceValues(forKeys:[.isRegularFileKey])).isRegularFile == true {
            let data = try Data(contentsOf:file)
            try require(!markers.contains(where:{data.range(of:$0) != nil}),"marker in disposable workspace file")
        }
    }
    print("PASS: captured output and disposable files including SQLite auxiliaries contain neither marker")
}
func inspectAccess(_ target: SecKeychain, _ bundle: URL) throws {
    let query: [String:Any] = [kSecClass as String:kSecClassGenericPassword, kSecAttrService as String:"com.metten.dummy-connection", kSecAttrAccount as String:"dummy", kSecAttrSynchronizable as String:false, kSecUseDataProtectionKeychain as String:false, kSecMatchSearchList as String:[target], kSecReturnRef as String:true]
    var result: CFTypeRef?; try ok(SecItemCopyMatching(query as CFDictionary,&result),"owned item reference inspection")
    let item = result as! SecKeychainItem
    var actual: SecKeychain?; try ok(SecKeychainItemCopyKeychain(item,&actual),"item location")
    try require(try path(actual!) == path(target),"item stored outside owned Keychain")
    var access: SecAccess?; try ok(SecKeychainItemCopyAccess(item,&access),"item access inspection")
    var entries: CFArray?; try ok(SecAccessCopyACLList(access!,&entries),"ACL list inspection")
    var creatorFound = false
    var decryptProtected = false
    for acl in entries! as! [SecACL] {
        let authorizations = SecACLCopyAuthorizations(acl) as! [String]
        var applications: CFArray?; var description: CFString?; var prompt = SecKeychainPromptSelector()
        try ok(SecACLCopyContents(acl,&applications,&description,&prompt),"ACL contents inspection")
        let apps = applications as? [SecTrustedApplication]
        // Apple legitimately grants all applications encryption, which reveals no secret.
        if apps == nil {
            try require(authorizations.allSatisfy { $0 == (kSecACLAuthorizationEncrypt as String) },"broad sensitive authorization found")
        } else {
            for application in apps! {
                var data: CFData?; try ok(SecTrustedApplicationCopyData(application,&data),"trusted application identity")
                let info = try PropertyListSerialization.propertyList(from:Data(contentsOf:bundle.appendingPathComponent("Contents/Info.plist")),format:nil) as! [String:Any]
                let executable = bundle.appendingPathComponent("Contents/MacOS/" + (info["CFBundleExecutable"] as! String)).path
                var expected: SecTrustedApplication?; try ok(SecTrustedApplicationCreateFromPath(executable,&expected),"creating-app reference")
                var expectedData: CFData?; try ok(SecTrustedApplicationCopyData(expected!,&expectedData),"creating-app identity")
                if CFEqual(data!,expectedData!) { creatorFound = true }
            }
            if authorizations.contains(kSecACLAuthorizationDecrypt as String) { decryptProtected = true }
        }
    }
    try require(creatorFound && decryptProtected,"creating-app identity or protected decrypt authorization missing (creator=\(creatorFound), protected=\(decryptProtected))")
    print("PASS: actual item location, creating-app ACL identity and protected sensitive authorizations")
}
// A runner-created item trusts the runner, so Metten must request access.
// Deny only a uniquely identified Metten/dummy prompt; never enter a password.
func denyOwnedPrompt(whileQuitting: Bool = false) throws {
    let deadline = Date().addingTimeInterval(8)
    while Date() < deadline {
        // NSWorkspace refreshes this list only when the main run loop runs.
        RunLoop.current.run(until:Date().addingTimeInterval(0.1))
        let agents = NSWorkspace.shared.runningApplications.filter { $0.bundleIdentifier == "com.apple.SecurityAgent" }
        let windows = agents.flatMap { attribute(AXUIElementCreateApplication($0.processIdentifier), kAXWindowsAttribute) as? [AXUIElement] ?? [] }
        let ownedPrompts = windows.filter { window in
            let visible = nodes(window).map { text($0) }.joined(separator:" ")
            return visible.localizedCaseInsensitiveContains("Metten") && visible.localizedCaseInsensitiveContains("dummy")
        }
        if ownedPrompts.count == 1 {
            let deny = nodes(ownedPrompts[0]).filter { (attribute($0,kAXRoleAttribute) as? String) == kAXButtonRole && text($0).trimmingCharacters(in:.whitespaces) == "Deny" }
            if deny.count == 1 {
                if whileQuitting {
                    try requestQuit()
                    try require(try !waitForChildExit(timeout:0.3),"Quit exited before pending native access was resolved")
                    print("PASS: guarded Quit retained the app during an actual pending Security prompt")
                }
                try require(AXUIElementPerformAction(deny[0],kAXPressAction as CFString) == .success,"owned prompt denial failed")
                print("PASS: denied actual Metten/dummy Security prompt")
                return
            }
        }
        usleep(100_000)
    }
    throw Failure(message:"BLOCKED: uniquely identified accessible Metten/dummy Deny control unavailable; no prompt modified")
}
func waitAccessRefusal() throws {
    let deadline = Date().addingTimeInterval(8)
    let categories = ["Denied —", "Canceled —", "Authentication failed —"]
    while Date() < deadline {
        for category in categories where appNodes().contains(where: { text($0).contains(category) }) {
            print("PASS: actual access refusal reported as " + category); return
        }
        usleep(100_000)
    }
    throw Failure(message:"actual prompt refusal did not produce a fixed access-refusal status")
}
func run() throws {
    try require(AXIsProcessTrusted(),"BLOCKED: native runner requires Accessibility permission")
    try require(CommandLine.arguments.count == 2,"supply finished .app path only")
    let bundle = URL(fileURLWithPath:CommandLine.arguments[1]).standardizedFileURL
    let before = try metadata(); baselineMetadata = before
    var directory = URL(fileURLWithPath:NSTemporaryDirectory()).appendingPathComponent("metten-credential-" + UUID().uuidString).resolvingSymlinksInPath()
    try fm.createDirectory(at:directory,withIntermediateDirectories:false,attributes:[.posixPermissions:0o700]); ownedDirectory = directory
    guard let resolved = realpath(directory.path,nil) else { throw Failure(message:"owned directory resolution failed") }; defer { free(resolved) }; directory = URL(fileURLWithPath:String(cString:resolved)); ownedDirectory = directory
    let directoryIdentity = try fm.attributesOfItem(atPath:directory.path)
    print("OWNED: disposable directory " + directory.path + "; device=\(directoryIdentity[.systemNumber]!) inode=\(directoryIdentity[.systemFileNumber]!)")
    let targetPath = directory.appendingPathComponent("dummy.keychain-db").path
    var target: SecKeychain?
    try ok(SecKeychainCreate(targetPath,UInt32(password.utf8.count),password,false,nil,&target),"create disposable Keychain")
    owned = target // Record ownership immediately after creation, before validation.
    try require(try path(target!) == targetPath,"unexpected owned Keychain path")
    try require(try metadata() == before,"isolation changed during creation")
    try launch(bundle,directory,target!)
    let originalPath = try path(target!)
    let originalAttrs = try fm.attributesOfItem(atPath:originalPath)
    try press("Start session"); try waitText("Running")
    try press("Advance test clock +1 second"); try waitText("Saved test clock: 1 seconds")
    try press("Pause now"); try waitText("Paused")
    try fill(a); try press("Save"); try waitText("Saved —"); try clearCheck()
    let currentPath = try path(target!)
    let currentAttrs = try fm.attributesOfItem(atPath:currentPath)
    print("DIAGNOSTIC: path unchanged=\(originalPath == currentPath); inode unchanged=\((originalAttrs[.systemFileNumber] as! NSNumber) == (currentAttrs[.systemFileNumber] as! NSNumber)); original file exists=\(fm.fileExists(atPath:originalPath))")
    try press("Save"); try waitText("Invalid input —")
    try fill(a); try press("Save"); try waitText("Already saved —"); try clearCheck()
    try check(b,"Mismatch —"); try check(a,"Readable —"); try inspectAccess(target!,bundle)
    var second: SecKeychain?
    try ok(SecKeychainCreate(directory.appendingPathComponent("fixture.keychain-db").path,UInt32(password.utf8.count),password,false,nil,&second),"create second disposable fixture")
    fixture = second
    let fixtureAdd: [String:Any] = [kSecClass as String:kSecClassGenericPassword,kSecAttrService as String:"com.metten.dummy-connection",kSecAttrAccount as String:"dummy",kSecAttrSynchronizable as String:false,kSecUseDataProtectionKeychain as String:false,kSecUseKeychain as String:second!,kSecValueData as String:Data(b.utf8)]
    try ok(SecItemAdd(fixtureAdd as CFDictionary,nil),"populate second disposable fixture")
    try quit(); try launch(bundle,directory,target!); try check(a,"Readable —")
    try fill(b); try press("Replace"); try waitText("Replaced —"); try clearCheck()
    try check(a,"Mismatch —"); try check(b,"Readable —")
    // Deny writes to this owned disposable file without changing Keychain ACLs.
    let protectedPath = try path(target!)
    let beforeFiles = Set(try fm.contentsOfDirectory(atPath:directory.path))
    try require(chflags(protectedPath,UInt32(UF_IMMUTABLE)) == 0,"could not protect disposable file for real replacement failure")
    do {
        defer { _ = chflags(protectedPath,0) }
        try fill(a); try press("Replace"); try waitText("Unavailable —"); try clearCheck()
        for name in try fm.contentsOfDirectory(atPath:directory.path) where !beforeFiles.contains(name) {
            try require(name.hasPrefix("dummy.keychain-db.sb-"),"unexpected failed-write artifact; retained")
            let file = directory.appendingPathComponent(name)
            let attrs = try fm.attributesOfItem(atPath:file.path)
            try require(attrs[.type] as? FileAttributeType == .typeRegular,"failed-write artifact is not regular")
            failureSidecars.append((file,(attrs[.systemFileNumber] as! NSNumber).uint64Value))
        }
    }
    try require(chflags(protectedPath,0) == 0,"failed to remove owned test write restriction")
    try check(b,"Readable —")
    print("PASS: real native replacement refusal preserved B")
    try ok(SecKeychainLock(target!),"lock disposable Keychain")
    try fill(a); try press("Save"); try waitText("Locked —"); try clearCheck()
    try check(b,"Locked —")
    try ok(SecKeychainUnlock(target!,UInt32(password.utf8.count),password,true),"explicit unlock")
    try check(b,"Readable —")
    try press("Remove"); try waitText("Removed —"); try check(b,"Missing —")
    try quit(); try launch(bundle,directory,target!); try check(b,"Missing —")
    try press("Remove"); try waitText("Missing —")
    try press("Check local credential use"); try waitText("Verification unavailable —")
    try quit(); try launch(bundle,directory,target!)
    try expectation(a,sequenceOverride:0); try press("Check local credential use"); try waitText("Verification unavailable —")
    try quit(); try launch(bundle,directory,target!)
    try expectation(a)
    try expectation(a,sequenceOverride:1)
    try press("Check local credential use"); try waitText("Verification unavailable —")
    try quit(); try launch(bundle,directory,target!)
    shutdown(socket,SHUT_WR)
    try press("Check local credential use"); try waitText("Verification unavailable —")
    try quit(); try launch(bundle,directory,target!)
    var partial = UInt32(100).bigEndian
    _ = withUnsafeBytes(of:&partial) { Darwin.write(socket,$0.baseAddress,4) }
    try press("Check local credential use"); try waitText("Verification unavailable —")
    try quit()
    print("PASS: guarded quit during incomplete verifier frame")
    let fixtureQuery: [String:Any] = [kSecClass as String:kSecClassGenericPassword,kSecAttrService as String:"com.metten.dummy-connection",kSecAttrAccount as String:"dummy",kSecAttrSynchronizable as String:false,kSecUseDataProtectionKeychain as String:false,kSecMatchSearchList as String:[fixture!],kSecReturnData as String:true]
    var fixtureData: CFTypeRef?; try ok(SecItemCopyMatching(fixtureQuery as CFDictionary,&fixtureData),"second fixture check")
    try require((fixtureData as! Data) == Data(b.utf8),"second fixture changed")
    print("PASS: second matching-identity fixture unchanged after main replacement/removal")
    let promptFixture: [String:Any] = [kSecClass as String:kSecClassGenericPassword,kSecAttrService as String:"com.metten.dummy-connection",kSecAttrAccount as String:"dummy",kSecAttrSynchronizable as String:false,kSecUseDataProtectionKeychain as String:false,kSecUseKeychain as String:target!,kSecValueData as String:Data(b.utf8)]
    try ok(SecItemAdd(promptFixture as CFDictionary,nil),"create runner-owned access-prompt fixture")
    try launch(bundle,directory,target!)
    try expectation(b)
    print("STEP: request dummy local-use access")
    try press("Check local credential use")
    print("STEP: locate owned Security prompt Deny control")
    try denyOwnedPrompt(); try waitAccessRefusal(); try clearCheck(); try quit()
    try launch(bundle,directory,target!)
    try expectation(b); try press("Check local credential use")
    try denyOwnedPrompt(whileQuitting:true)
    try require(try waitForChildExit(timeout:8),"Quit did not finish after native access denial")
    print("PASS: guarded Quit finished after actual native access denial")
    try launch(bundle,directory,target!,isolatedSetup:false)
    try fill(a); try press("Save"); try waitText("Unavailable —"); try clearCheck(); try quit()
    try scan(directory)
    for relative in ["Library/WebKit/com.metten.foundation", "Library/Caches/com.metten.foundation", "Library/HTTPStorages/com.metten.foundation"] {
        let browserDirectory = fm.homeDirectoryForCurrentUser.appendingPathComponent(relative)
        if fm.fileExists(atPath:browserDirectory.path) { try scan(browserDirectory); print("PASS: marker scan of " + relative) }
        else { print("ABSENT: " + relative) }
    }
    try require(try metadata() == before,"isolation changed during app sequence")
    print("PASS: default/search-list metadata unchanged")
}
if CommandLine.arguments.dropFirst() == ["--self-test"] {
    do { try recoverySelfTest(); exit(0) }
    catch { print("FAIL: process-only recovery self-test"); exit(1) }
}
var failed = false
do { try run() } catch let error as Failure { print("FAIL/BLOCKED: " + error.message); failed = true } catch { print("FAIL/BLOCKED: native runner error (details suppressed)"); failed = true }
if child != 0 { do { try quit() } catch { failed = true; print("CLEANUP: guarded exit not confirmed") } }
if child != 0 {
    failed = true
    print("CLEANUP PENDING: runner retains ownership; waiting for actual app exit")
    _ = awaitRetainedChildExit()
    print("CLEANUP: late app exit confirmed; verification remains failed")
}
for (file, inode) in failureSidecars {
    do {
        let attrs = try fm.attributesOfItem(atPath:file.path)
        try require((attrs[.systemFileNumber] as! NSNumber).uint64Value == inode,"failed-write artifact identity changed")
        try require(chflags(file.path,0) == 0,"failed to clear owned artifact test flag")
        try fm.removeItem(at:file)
    } catch { print("CLEANUP FAILED: recorded failed-write artifact retained"); exit(1) }
}
if let target = fixture { if SecKeychainDelete(target) != 0 { print("CLEANUP FAILED: fixture retained"); exit(1) } }
if let target = owned {
    let status = SecKeychainDelete(target)
    if status != 0 { print("CLEANUP FAILED: owned Keychain deletion OSStatus \(status); directory retained"); exit(1) }
    print("PASS: deleted exact owned disposable Keychain")
}
if let directory = ownedDirectory {
    do { try fm.removeItem(at:directory); print("PASS: removed owned disposable directory") }
    catch { print("CLEANUP FAILED: owned directory retained"); exit(1) }
}
if let baseline = baselineMetadata {
    do { try require(try metadata() == baseline,"default/search-list changed after cleanup"); print("PASS: isolation metadata unchanged after cleanup") }
    catch { print("FAIL: isolation metadata changed or unavailable after cleanup"); failed = true }
}
exit(failed ? 1 : 0)
