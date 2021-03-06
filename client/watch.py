#!/usr/bin/python3 -B
from http.server import HTTPServer, SimpleHTTPRequestHandler
from functools import partial
import subprocess
import os
from threading import Thread

import sys
sys.path.append("../util/filewatcher")
try:
    import filewatcher
except:
    print(f"Error: you need to run the script from within '{sys.path[0]}/'.")
    exit(1)

port = 8080
serve_dir = "static"

def build_wasm():
    print("===== BUILDING WASM =====")
    LockableRequestHandler.lock = True
    result = subprocess.run(["wasm-pack", "build", "--dev", "--target", "web", "--no-typescript"])
    if result.returncode == 0:
        os.replace("pkg/client_bg.wasm", f"{serve_dir}/client_bg.wasm")
        os.replace("pkg/client.js", f"{serve_dir}/client.js")
        LockableRequestHandler.lock = False
    else:
        print("===== ERROR BUILDING WASM =====")

def watch():
    filewatcher.watch(
        build_wasm, 1,
        watch_dirs=["src", "../common/src"],
        watch_files=["Cargo.toml", "../common/Cargo.toml"]
    )

class LockableRequestHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        if not LockableRequestHandler.lock:
            super().do_GET()

LockableRequestHandler.lock = False

if __name__ == "__main__":
    watcher = Thread(target=watch, daemon=True)
    watcher.start()
    Handler = partial(LockableRequestHandler, directory=serve_dir)
    server = HTTPServer(("", port), Handler)
    server.serve_forever()
