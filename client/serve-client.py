#!/usr/bin/python3
from http.server import HTTPServer, SimpleHTTPRequestHandler
from functools import partial
import subprocess
from threading import Thread
import time
import os
from glob import glob
from pathlib import Path
import hashlib

port = 8080
file_watcher_delay_secs = 1
serve_dir = "static"
watch_dirs = ["src", "../common/src"]

def build_wasm():
    print("===== BUILDING WASM =====")
    result = subprocess.run(["wasm-pack", "build", "--target", "web"])
    if result.returncode == 0:
        os.replace("pkg/client_bg.wasm", f"{serve_dir}/client_bg.wasm")
        os.replace("pkg/client.js", f"{serve_dir}/client.js")
    else:
        print("===== ERROR BUILDING WASM =====")

def watch_files():
    prev_checksums = []
    while True:
        files = []
        for watchdir in watch_dirs:
            files += sorted(list(filter(lambda i: os.path.isfile(i), glob(f"{watchdir}/**", recursive=True))))
        files.append("Cargo.toml")
        files.append("../common/Cargo.toml")
        curr_checksums = list(map(lambda i: f"{i}: {hash(Path(i).read_text())}", files))
        if curr_checksums != prev_checksums:
            build_wasm()
        prev_checksums = curr_checksums
        time.sleep(file_watcher_delay_secs)


if __name__ == "__main__":
    print(f"===== WATCHING FILES =====")
    watcher = Thread(target=watch_files, daemon=True)
    watcher.start()
    Handler = partial(SimpleHTTPRequestHandler, directory=serve_dir)
    server = HTTPServer(("", port), Handler)
    server.serve_forever()
