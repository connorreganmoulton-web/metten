"""Disposable-Git regression checks for staged snapshot isolation and failures."""
from pathlib import Path
import os
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "scripts/check-staged.py"


class StagedCheckpoint(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(prefix="metten-staged-test-")
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.git("init", "--quiet")
        (self.root / "value.txt").write_text("staged\n")
        self.git("add", "value.txt")

    def git(self, *args):
        return subprocess.check_output(["git", *args], cwd=self.root)

    def run_check(self, code):
        original_index = (self.root / ".git/index").read_bytes()
        index = self.git("ls-files", "--stage")
        status = self.git("status", "--porcelain=v1", "--untracked-files=all")
        result = subprocess.run(
            [sys.executable, str(SCRIPT), sys.executable, "-c", code],
            cwd=self.root, text=True, capture_output=True,
        )
        self.assertEqual(index, self.git("ls-files", "--stage"))
        self.assertEqual(status, self.git("status", "--porcelain=v1", "--untracked-files=all"))
        self.assertEqual((self.root / ".git/index").read_bytes(), original_index)
        return result

    def test_only_staged_files_are_visible_and_export_is_removed(self):
        (self.root / "value.txt").write_text("unstaged\n")
        (self.root / "missing-dependency.txt").write_text("not staged\n")
        result = self.run_check(
            "from pathlib import Path; "
            "assert Path('value.txt').read_text() == 'staged\\n'; "
            "assert not Path('missing-dependency.txt').exists(); "
            "print(Path.cwd())"
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(Path(result.stdout.splitlines()[-1]).exists())
        self.assertEqual((self.root / "value.txt").read_text(), "unstaged\n")

    def test_missing_staged_dependency_fails(self):
        (self.root / "missing-dependency.txt").write_text("not staged\n")
        result = self.run_check("from pathlib import Path; Path('missing-dependency.txt').read_text()")
        self.assertNotEqual(result.returncode, 0)

    def test_whitespace_failure_stops_before_command(self):
        (self.root / "value.txt").write_text("bad trailing whitespace \n")
        self.git("add", "value.txt")
        result = self.run_check("print('COMMAND_RAN')")
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("COMMAND_RAN", result.stdout)
        self.assertIn("trailing whitespace", result.stdout)

    def test_command_failure_is_preserved_and_export_is_removed(self):
        result = self.run_check("from pathlib import Path; import sys; print(Path.cwd()); sys.exit(7)")
        self.assertEqual(result.returncode, 7)
        self.assertFalse(Path(result.stdout.splitlines()[-1]).exists())

    def test_export_command_cannot_inherit_the_source_index(self):
        index_path = self.root / ".git/index"
        original = index_path.read_bytes()
        result = subprocess.run(
            [sys.executable, str(SCRIPT), sys.executable, "-c",
             "import os; assert 'GIT_INDEX_FILE' not in os.environ"],
            cwd=self.root, env=dict(os.environ, GIT_INDEX_FILE=str(index_path)),
            text=True, capture_output=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(index_path.read_bytes(), original)


if __name__ == "__main__":
    unittest.main()
