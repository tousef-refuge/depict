from .args import SubArgs
from py import print_error
import os

def process_path(func, sysargs):
    root = sysargs.root
    if os.path.isfile(root):
        func(SubArgs(sysargs, root))
    elif os.path.isdir(root):
        _dir_walk(func, sysargs)
    else:
        print_error("Path does not exist")

def _dir_walk(func, sysargs):
    root = sysargs.root
    for current_path, dirs, files in os.walk(root):
        for file in files:
            if file.lower().endswith(".png"):
                full_path = os.path.join(current_path, file)
                try:
                    func(SubArgs(sysargs, full_path))
                except Exception as e:
                    print(f"Error processing {full_path}: {e}")