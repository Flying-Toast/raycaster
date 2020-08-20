import time
import os
from glob import glob
from pathlib import Path

def watch(on_change, delay_secs, watch_dirs=[], watch_files=[]):
    prev_checksums = []
    while True:
        files = []
        for watchdir in watch_dirs:
            files += sorted(list(filter(lambda i: os.path.isfile(i), glob(f"{watchdir}/**", recursive=True))))
        files += watch_files
        curr_checksums = list(map(lambda i: f"{i}: {hash(Path(i).read_text())}", files))
        if curr_checksums != prev_checksums:
            on_change()
        prev_checksums = curr_checksums
        time.sleep(delay_secs)
