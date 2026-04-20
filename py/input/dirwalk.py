from .filters import filter_init
from .processing import process
from py import print_error
import os

def process_path(func, sysargs):
    filter_init(sysargs)
    root = sysargs["path"]
    if os.path.isfile(root):
        process(func, sysargs, root)
    elif os.path.isdir(root):
        _dir_walk(func, sysargs)
    else:
        print_error("Path does not exist")

def _dir_walk(func, sysargs):
    root = sysargs["path"]
    for current_path, dirs, files in os.walk(root):
        for file in files:
            full_path = os.path.join(current_path, file)
            process(func, sysargs, full_path)
