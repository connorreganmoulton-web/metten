"""Run one check against staged files, independently of the working directory."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def main(command):
    if not command:
        print("Usage: python3 scripts/check-staged.py COMMAND [ARG ...]", file=sys.stderr)
        return 2
    root = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()
    source_index = Path(subprocess.check_output(
        ["git", "rev-parse", "--path-format=absolute", "--git-path", "index"], text=True,
    ).strip())
    with tempfile.TemporaryDirectory(prefix="metten-staged-") as directory:
        source = Path(directory) / "source"
        source.mkdir()
        index_env = dict(os.environ, GIT_INDEX_FILE=str(Path(directory) / "index"))
        if source_index.exists():
            shutil.copyfile(source_index, index_env["GIT_INDEX_FILE"])
        subprocess.run(["git", "diff", "--cached", "--check"], cwd=root, env=index_env, check=True)
        tree = subprocess.check_output(["git", "write-tree"], cwd=root, env=index_env, text=True).strip()
        print(f"Checking staged tree {tree}", flush=True)
        subprocess.run(["git", "read-tree", tree], cwd=root, env=index_env, check=True)
        subprocess.run(
            ["git", "checkout-index", "--all", f"--prefix={source}/"],
            cwd=root, env=index_env, check=True,
        )
        command_env = dict(os.environ)
        for name in ("GIT_INDEX_FILE", "GIT_DIR", "GIT_WORK_TREE", "GIT_COMMON_DIR", "GIT_PREFIX"):
            command_env.pop(name, None)
        subprocess.run(command, cwd=source, env=command_env, check=True)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except subprocess.CalledProcessError as error:
        sys.exit(error.returncode if error.returncode > 0 else 1)
