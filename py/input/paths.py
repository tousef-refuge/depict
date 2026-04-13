from .args import SubArgs
from .filters import filter_init, skip_file
from py import print_error
import os

def process_path(func, sysargs):
    filter_init(sysargs)
    root = sysargs["path"]
    if os.path.isfile(root):
        func(SubArgs(sysargs, root))
    elif os.path.isdir(root):
        _dir_walk(func, sysargs)
    else:
        print_error("Path does not exist")

def _dir_walk(func, sysargs):
    root = sysargs["path"]
    for current_path, dirs, files in os.walk(root):
        for file in files:
            if file.lower().endswith(".png"):
                full_path = os.path.join(current_path, file)
                if skip_file(full_path):
                    continue

                try:
                    func(SubArgs(sysargs, full_path))
                except Exception as e:
                    print(f"Error processing {full_path}: {e}")
