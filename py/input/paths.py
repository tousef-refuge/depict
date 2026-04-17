from .filters import filter_init, skip_file, VALID_FILE_EXTS
from PIL import Image
from py import print_error
import os

def process_path(func, sysargs):
    filter_init(sysargs)
    root = sysargs["path"]
    if os.path.isfile(root):
        _run_func(func, sysargs, root)
    elif os.path.isdir(root):
        _dir_walk(func, sysargs)
    else:
        print_error("Path does not exist")

def _dir_walk(func, sysargs):
    root = sysargs["path"]
    for current_path, dirs, files in os.walk(root):
        for file in files:
            full_path = os.path.join(current_path, file)
            _run_func(func, sysargs, full_path)

def _run_func(func, sysargs, path):
    if skip_file(path) or not path.lower().endswith(VALID_FILE_EXTS):
        return

    img = Image.open(path).convert("RGBA")
    result = func(sysargs, img)
    if not path.lower().endswith(".png"):
        result = result.convert("RGB")
    result.save(path)