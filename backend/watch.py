#!/usr/bin/python3 -B
import subprocess

import sys
sys.path.append("../util/filewatcher")
import filewatcher

def on_change():
    if on_change.running_process is not None:
        on_change.running_process.terminate()
        on_change.running_process = None
    print("===== BUILDING... =====")
    result = subprocess.run(["cargo", "build"])
    if result.returncode == 0:
        on_change.running_process = subprocess.Popen(["cargo", "run"])
    else:
        print("===== ERROR RUNNING `cargo build` =====")

on_change.running_process = None

if __name__ == "__main__":
    filewatcher.watch(
        on_change, 1,
        watch_dirs=["src", "../common/src"],
        watch_files=["Cargo.toml", "../common/Cargo.toml"]
    )
