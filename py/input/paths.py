from .args import SubArgs
from .filters import filter_init, skip_file
from py import print_error
import os

def process_path(func, sysargs):
    filter_init(sysargs)
    root = sysargs["path"]
    if os.path.isfile(root):
        if is_valid_file(root) and not skip_file(root):
            func(SubArgs(sysargs, root))
    elif os.path.isdir(root):
        _dir_walk(func, sysargs)
    else:
        print_error("Path does not exist")

def _dir_walk(func, sysargs):
    root = sysargs["path"]
    for current_path, dirs, files in os.walk(root):
        for file in files:
            if is_valid_file(file):
                full_path = os.path.join(current_path, file)
                if skip_file(full_path):
                    continue
                func(SubArgs(sysargs, full_path))

def is_valid_file(path):
    return path.lower().endswith(".png")
