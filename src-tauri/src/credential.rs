//! Dummy-only file-Keychain proof. No destination or identity comes from the screen.
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    boolean::CFBoolean,
    data::CFData,
    dictionary::CFDictionary,
    string::CFString,
};
use security_framework_sys::{base::SecKeychainRef, item::*, keychain::*};
use serde::Deserialize;
use std::{
    ffi::CString,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::{fs::MetadataExt, net::UnixStream},
    },
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

// These public legacy lifecycle functions are absent from security-framework-sys.
unsafe extern "C" {
    fn SecKeychainGetPath(keychain: SecKeychainRef, length: *mut u32, path: *mut i8) -> i32;
    fn SecKeychainGetStatus(keychain: SecKeychainRef, status: *mut u32) -> i32;
}
use security_framework_sys::keychain_item::{
    SecItemAdd, SecItemCopyMatching, SecItemDelete, SecItemUpdate,
};
const UNAVAILABLE: &str = "Unavailable — reopen with the isolated dummy verification runner.";
const LOCKED: &str =
    "Locked — explicitly unlock the disposable Keychain in the runner, then retry.";
const CHANNEL: &str =
    "Verification unavailable — prepare one fresh expectation in the runner, then retry.";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Bootstrap {
    path: PathBuf,
    device: u64,
    inode: u64,
    password: String,
    session: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Expectation {
    session: String,
    sequence: u64,
    phase: String,
    expected: String,
}
#[derive(Default)]
struct Verifier {
    pending: Option<String>,
    failed: bool,
}
pub struct Credentials {
    target: Mutex<Option<Bootstrap>>,
    verifier: Arc<Mutex<Verifier>>,
    pub operation: Mutex<()>,
}
fn receive(
    stream: &mut UnixStream,
    bytes: &mut [u8],
    deadline: Instant,
) -> Result<(), &'static str> {
    let mut offset = 0;
    while offset < bytes.len() {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .ok_or(CHANNEL)?;
        stream
            .set_read_timeout(Some(remaining))
            .map_err(|_| CHANNEL)?;
        let n = stream.read(&mut bytes[offset..]).map_err(|_| CHANNEL)?;
        if n == 0 {
            return Err(CHANNEL);
        }
        offset += n;
    }
    Ok(())
}
fn frame(stream: &mut UnixStream) -> Result<Vec<u8>, &'static str> {
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut length = [0; 4];
    receive(stream, &mut length, deadline)?;
    let length = u32::from_be_bytes(length) as usize;
    if !(1..=4096).contains(&length) {
        return Err(CHANNEL);
    }
    let mut bytes = vec![0; length];
    receive(stream, &mut bytes, deadline)?;
    Ok(bytes)
}
fn observe(mut stream: UnixStream, session: String, state: std::sync::Weak<Mutex<Verifier>>) {
    let mut sequence = 0;
    loop {
        let mut poll = libc::pollfd {
            fd: stream.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let ready = unsafe { libc::poll(&mut poll, 1, 200) };
        let Some(state) = state.upgrade() else {
            return;
        };
        if ready == 0 {
            continue;
        }
        let received = if ready > 0 {
            frame(&mut stream)
                .ok()
                .and_then(|b| serde_json::from_slice::<Expectation>(&b).ok())
        } else {
            None
        };
        let Ok(mut state) = state.lock() else {
            return;
        };
        let valid = received.filter(|e| {
            !state.failed
                && state.pending.is_none()
                && e.session == session
                && e.sequence == sequence + 1
                && e.phase == "local-use"
                && dummy(&e.expected)
        });
        match valid {
            Some(e) => {
                sequence += 1;
                state.pending = Some(e.expected);
                // Acknowledge receipt before the runner may press the UI command.
                if stream.write_all(&[1]).is_err() {
                    state.failed = true;
                    state.pending = None;
                    return;
                }
            }
            None => {
                state.failed = true;
                state.pending = None;
                let _ = stream.write_all(&[0]);
                return;
            }
        }
    }
}
fn map_status(status: i32) -> Result<(), &'static str> {
    match status {
        0 => Ok(()),
        -25299 => Err("Already saved — use Replace to change the dummy credential."),
        -25300 => Err("Missing — save a dummy credential first."),
        -128 => Err("Canceled — access was canceled; retry when ready."),
        -25293 => Err("Authentication failed — check the disposable Keychain setup in the runner."),
        -25308 => {
            Err("Denied — macOS requires access; check the disposable Keychain prompt and retry.")
        }
        -25243 => Err("Denied — macOS refused access to the disposable Keychain."),
        _ => Err(UNAVAILABLE),
    }
}
fn dummy(value: &str) -> bool {
    value.starts_with("metten-dummy-")
        && (20..=128).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
}
impl Bootstrap {
    fn validate(&self) -> Result<(), &'static str> {
        let parent = self.path.parent().ok_or(UNAVAILABLE)?;
        let name = parent
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or(UNAVAILABLE)?;
        let path = self.path.to_str().ok_or(UNAVAILABLE)?;
        if path == "/Library/Keychains/System.keychain"
            || path.contains("/login.keychain")
            || !name.starts_with("metten-credential-")
            || self.path.file_name().and_then(|s| s.to_str()) != Some("dummy.keychain-db")
            || self.path.canonicalize().ok().as_ref() != Some(&self.path)
        {
            return Err(UNAVAILABLE);
        }
        let dir = std::fs::symlink_metadata(parent).map_err(|_| UNAVAILABLE)?;
        let file = std::fs::symlink_metadata(&self.path).map_err(|_| UNAVAILABLE)?;
        // Canonical equality rejects path aliases; identity rejects replaced files.
        if !dir.is_dir()
            || dir.mode() & 0o777 != 0o700
            || dir.uid() != unsafe { libc::geteuid() }
            || !file.is_file()
            || file.uid() != dir.uid()
            || file.dev() != self.device
            || file.ino() != self.inode
            || file.nlink() != 1
        {
            return Err(UNAVAILABLE);
        }
        Ok(())
    }
    fn record_native_write(&mut self) -> Result<(), &'static str> {
        // Security atomically replaces file-Keychains on successful writes.
        // Accept a new file identity only immediately after our own successful API call.
        let file = std::fs::symlink_metadata(&self.path).map_err(|_| UNAVAILABLE)?;
        if !file.is_file()
            || file.uid() != unsafe { libc::geteuid() }
            || file.nlink() != 1
            || file.dev() != self.device
        {
            return Err(UNAVAILABLE);
        }
        self.inode = file.ino();
        self.validate()
    }
    fn open(&self) -> Result<CFType, &'static str> {
        self.validate()?;
        let path =
            CString::new(self.path.as_os_str().as_encoded_bytes()).map_err(|_| UNAVAILABLE)?;
        let mut raw = std::ptr::null_mut();
        unsafe {
            map_status(SecKeychainOpen(path.as_ptr(), &mut raw))?;
            if raw.is_null() {
                return Err(UNAVAILABLE);
            }
            let owned = CFType::wrap_under_create_rule(raw.cast());
            let mut buffer = [0u8; 4096];
            let mut length = buffer.len() as u32;
            map_status(SecKeychainGetPath(
                raw,
                &mut length,
                buffer.as_mut_ptr().cast(),
            ))?;
            if length as usize > buffer.len()
                || buffer[..length as usize]
                    .strip_suffix(&[0])
                    .unwrap_or(&buffer[..length as usize])
                    != self.path.as_os_str().as_encoded_bytes()
            {
                return Err(UNAVAILABLE);
            }
            self.validate()?;
            Ok(owned)
        }
    }
}
impl Credentials {
    pub fn begin_operation<'a>(
        &'a self,
        quitting: &std::sync::atomic::AtomicBool,
    ) -> Result<std::sync::MutexGuard<'a, ()>, &'static str> {
        let guard = self.operation.lock().map_err(|_| UNAVAILABLE)?;
        if quitting.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("Quitting — credential action refused.");
        }
        Ok(guard)
    }
    pub fn setup() -> Self {
        let setup = (|| -> Result<(Bootstrap, UnixStream), &'static str> {
            if std::env::var("METTEN_VERIFY_FD").ok().as_deref() != Some("3") {
                return Err(UNAVAILABLE);
            }
            let mut stream = unsafe { UnixStream::from_raw_fd(3) };
            stream.peer_addr().map_err(|_| UNAVAILABLE)?;
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .map_err(|_| UNAVAILABLE)?;
            let mut target: Bootstrap =
                serde_json::from_slice(&frame(&mut stream)?).map_err(|_| UNAVAILABLE)?;
            if target.password.len() < 32
                || target.password.len() > 128
                || target.session.len() != 32
            {
                return Err(UNAVAILABLE);
            }
            let keychain = target.open()?;
            unsafe {
                map_status(SecKeychainUnlock(
                    keychain.as_CFTypeRef() as _,
                    target.password.len() as u32,
                    target.password.as_ptr().cast(),
                    1,
                ))?;
            }
            unlocked(&keychain)?;
            target.password = String::new();
            Ok((target, stream))
        })();
        let verifier = Arc::new(Mutex::new(Verifier::default()));
        let target = match setup {
            Ok((target, stream)) => {
                let session = target.session.clone();
                let weak = Arc::downgrade(&verifier);
                std::thread::spawn(move || observe(stream, session, weak));
                Some(target)
            }
            Err(_) => None,
        };
        Self {
            target: Mutex::new(target),
            verifier,
            operation: Mutex::new(()),
        }
    }
    // Receipt and framing finish before the UI command; this never waits on input.
    pub fn expectation(&self) -> Result<String, &'static str> {
        let mut v = self.verifier.lock().map_err(|_| CHANNEL)?;
        if v.failed {
            return Err(CHANNEL);
        }
        v.pending.take().ok_or(CHANNEL)
    }
    pub fn execute(
        &self,
        operation: &str,
        value: Option<&str>,
        expected: Option<&str>,
    ) -> Result<&'static str, &'static str> {
        let mut target_guard = self.target.lock().map_err(|_| UNAVAILABLE)?;
        let target = target_guard.as_mut().ok_or(UNAVAILABLE)?;
        let keychain = target.open()?;
        unlocked(&keychain)?;
        unsafe {
            let mut pairs = vec![
                (
                    CFString::wrap_under_get_rule(kSecClass).as_CFType(),
                    CFString::wrap_under_get_rule(kSecClassGenericPassword).as_CFType(),
                ),
                (
                    CFString::wrap_under_get_rule(kSecAttrService).as_CFType(),
                    CFString::new("com.metten.dummy-connection").as_CFType(),
                ),
                (
                    CFString::wrap_under_get_rule(kSecAttrAccount).as_CFType(),
                    CFString::new("dummy").as_CFType(),
                ),
                (
                    CFString::wrap_under_get_rule(kSecAttrSynchronizable).as_CFType(),
                    CFBoolean::false_value().as_CFType(),
                ),
                (
                    CFString::wrap_under_get_rule(kSecUseDataProtectionKeychain).as_CFType(),
                    CFBoolean::false_value().as_CFType(),
                ),
            ];
            if operation == "save" {
                let value = value
                    .filter(|v| dummy(v))
                    .ok_or("Invalid input — enter a bounded dummy verification value.")?;
                pairs.push((
                    CFString::wrap_under_get_rule(kSecUseKeychain).as_CFType(),
                    keychain.clone(),
                ));
                pairs.push((
                    CFString::wrap_under_get_rule(kSecValueData).as_CFType(),
                    CFData::from_buffer(value.as_bytes()).as_CFType(),
                ));
                map_status(SecItemAdd(
                    CFDictionary::from_CFType_pairs(&pairs).as_concrete_TypeRef(),
                    std::ptr::null_mut(),
                ))?;
                target.record_native_write()?;
                return Ok(
                    "Saved — local readability has not been checked. No exchange is connected.",
                );
            }
            pairs.push((
                CFString::wrap_under_get_rule(kSecMatchSearchList).as_CFType(),
                CFArray::from_CFTypes(&[keychain]).as_CFType(),
            ));
            match operation {
                "replace" => {
                    let value = value
                        .filter(|v| dummy(v))
                        .ok_or("Invalid input — enter a bounded dummy verification value.")?;
                    let update = CFDictionary::from_CFType_pairs(&[(
                        CFString::wrap_under_get_rule(kSecValueData),
                        CFData::from_buffer(value.as_bytes()),
                    )]);
                    map_status(SecItemUpdate(
                        CFDictionary::from_CFType_pairs(&pairs).as_concrete_TypeRef(),
                        update.as_concrete_TypeRef(),
                    ))?;
                    target.record_native_write()?;
                    Ok("Replaced — check local credential use. No exchange is connected.")
                }
                "remove" => {
                    map_status(SecItemDelete(
                        CFDictionary::from_CFType_pairs(&pairs).as_concrete_TypeRef(),
                    ))?;
                    target.record_native_write()?;
                    Ok("Removed — no exchange is connected.")
                }
                "check" => {
                    pairs.push((
                        CFString::wrap_under_get_rule(kSecReturnData).as_CFType(),
                        CFBoolean::true_value().as_CFType(),
                    ));
                    let mut result = std::ptr::null();
                    map_status(SecItemCopyMatching(
                        CFDictionary::from_CFType_pairs(&pairs).as_concrete_TypeRef(),
                        &mut result,
                    ))?;
                    let data = CFType::wrap_under_create_rule(result);
                    let data = data.downcast::<CFData>().ok_or(UNAVAILABLE)?;
                    // The observer sees only bytes returned by this actual trusted-use read.
                    if expected.is_some_and(|e| data.bytes() == e.as_bytes()) {
                        Ok(
                            "Readable — dummy local-use comparison passed. No exchange is connected.",
                        )
                    } else {
                        Err("Mismatch — local-use comparison failed; the saved item is unchanged.")
                    }
                }
                _ => Err("Invalid request — choose a supported credential action."),
            }
        }
    }
}
fn unlocked(keychain: &CFType) -> Result<(), &'static str> {
    let mut status = 0;
    unsafe {
        map_status(SecKeychainGetStatus(
            keychain.as_CFTypeRef() as _,
            &mut status,
        ))?;
    }
    if status & 1 == 0 { Err(LOCKED) } else { Ok(()) }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn isolated_state() -> Credentials {
        Credentials {
            target: Mutex::new(None),
            verifier: Arc::new(Mutex::new(Verifier::default())),
            operation: Mutex::new(()),
        }
    }
    #[test]
    fn queued_actions_refuse_after_quit_and_quit_waits_for_current_work() {
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            mpsc,
        };
        let credentials = isolated_state();
        let quitting = AtomicBool::new(false);
        std::thread::scope(|scope| {
            let active = credentials.begin_operation(&quitting).unwrap();
            let (started, waiting) = mpsc::channel();
            let mut queued = Vec::new();
            for _ in ["save", "replace", "remove", "check"] {
                let started = started.clone();
                let credentials = &credentials;
                let quitting = &quitting;
                queued.push(scope.spawn(move || {
                    started.send(()).unwrap();
                    assert_eq!(
                        credentials.begin_operation(quitting).err(),
                        Some("Quitting — credential action refused.")
                    );
                }));
            }
            for _ in 0..4 {
                waiting.recv().unwrap();
            }
            quitting.store(true, Ordering::SeqCst);
            let (finished, stopped) = mpsc::channel();
            let credentials = &credentials;
            let quit = scope.spawn(move || {
                let _pending = credentials.operation.lock().unwrap();
                finished.send(()).unwrap();
            });
            assert!(stopped.try_recv().is_err());
            assert!(credentials.operation.try_lock().is_err());
            drop(active);
            for command in queued {
                command.join().unwrap();
            }
            quit.join().unwrap();
            stopped.recv().unwrap();
        });
        assert!(credentials.begin_operation(&quitting).is_err());
    }
    #[test]
    fn target_rejects_unexplained_replacement_aliases_and_shared_files() {
        use std::os::unix::fs::{PermissionsExt, symlink};
        let directory = std::env::temp_dir().canonicalize().unwrap().join(format!(
            "metten-credential-identity-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = directory.join("dummy.keychain-db");
        std::fs::write(&path, []).unwrap();
        let metadata = std::fs::metadata(&path).unwrap();
        let target = Bootstrap {
            path: path.clone(),
            device: metadata.dev(),
            inode: metadata.ino(),
            password: String::new(),
            session: String::new(),
        };
        assert!(target.validate().is_ok());
        let alias = directory.join("alias");
        std::fs::hard_link(&path, &alias).unwrap();
        assert!(target.validate().is_err());
        std::fs::remove_file(&alias).unwrap();
        std::fs::rename(&path, &alias).unwrap();
        std::fs::write(&path, []).unwrap();
        assert!(target.validate().is_err());
        std::fs::remove_file(&path).unwrap();
        symlink(&alias, &path).unwrap();
        assert!(target.validate().is_err());
        std::fs::remove_file(&path).unwrap();
        std::fs::rename(&alias, &path).unwrap();
        assert!(target.validate().is_ok());
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(target.validate().is_err());
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::remove_file(&path).unwrap();
        std::fs::remove_dir(&directory).unwrap();
    }
    #[test]
    fn input_and_failures_are_bounded() {
        assert!(dummy("metten-dummy-12345678"));
        assert!(!dummy("real credential"));
        assert!(!dummy(&"metten-dummy-".repeat(20)));
        for code in [-128, -25293, -25308, -25243, -25300, -25299, -1] {
            assert!(map_status(code).is_err());
        }
        assert!(map_status(0).is_ok());
    }
    #[test]
    fn channel_frames_are_bounded_and_closed_input_fails() {
        use std::io::Write;
        let (mut a, mut b) = UnixStream::pair().unwrap();
        a.write_all(&4097u32.to_be_bytes()).unwrap();
        assert!(frame(&mut b).is_err());
        drop(a);
        assert!(frame(&mut b).is_err());
    }
    #[test]
    fn expectation_is_received_once_and_duplicate_is_rejected() {
        use std::io::{Read, Write};
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        sender
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let state = Arc::new(Mutex::new(Verifier::default()));
        let weak = Arc::downgrade(&state);
        let thread = std::thread::spawn(move || observe(receiver, "session".into(), weak));
        let send = |sender: &mut UnixStream, sequence: u64| {
            let bytes = serde_json::to_vec(&serde_json::json!({"session":"session", "sequence":sequence, "phase":"local-use", "expected":"metten-dummy-12345678"})).unwrap();
            sender
                .write_all(&(bytes.len() as u32).to_be_bytes())
                .unwrap();
            sender.write_all(&bytes).unwrap();
            let mut ack = [255];
            sender.read_exact(&mut ack).unwrap();
            ack[0]
        };
        assert_eq!(send(&mut sender, 1), 1);
        assert!(state.lock().unwrap().pending.take().is_some());
        assert!(state.lock().unwrap().pending.take().is_none());
        assert_eq!(send(&mut sender, 1), 0);
        assert!(state.lock().unwrap().failed);
        thread.join().unwrap();
    }
}
