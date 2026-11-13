"""Process-only regression check; never launches Metten or accesses Keychain."""
from pathlib import Path
import subprocess
import tempfile
import time
import unittest


class CredentialRunnerRecovery(unittest.TestCase):
    def test_progress_is_visible_before_exit_and_late_exit_is_recovered(self):
        root = Path(__file__).resolve().parents[1]
        with tempfile.TemporaryDirectory(prefix="metten-runner-check-") as directory:
            binary = Path(directory) / "runner"
            subprocess.run(
                ["swiftc", "-suppress-warnings", str(root / "scripts/verify-credential.swift"), "-o", str(binary)],
                check=True, capture_output=True, timeout=60,
            )
            output = Path(directory) / "progress.log"
            with output.open("wb") as stream:
                process = subprocess.Popen([str(binary), "--self-test"], stdin=subprocess.PIPE, stdout=stream, stderr=subprocess.STDOUT)
                try:
                    deadline = time.monotonic() + 5
                    while b"SELF-TEST: pending child retained" not in output.read_bytes():
                        self.assertIsNone(process.poll(), output.read_text())
                        self.assertLess(time.monotonic(), deadline, "progress stayed buffered while child was pending")
                        time.sleep(0.02)
                    self.assertIsNone(process.poll(), "runner exited before the gate was released")
                finally:
                    # Release only this process-only test; no native app is involved.
                    process.communicate(input=b"\x01", timeout=5)
                self.assertEqual(process.returncode, 0, output.read_text())
            result = output.read_text()
            self.assertIn("closed private channel write caught without terminating runner", result)
            self.assertIn("late child exit recovered, channels closed, unsuccessful exit stays failed", result)
            print(result, end="")


if __name__ == "__main__":
    unittest.main()
