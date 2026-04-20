from .filters import filter_init, skip_file
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

# noinspection PyUnresolvedReferences
def _run_func(func, sysargs, path):
    if skip_file(path, func):
        print(f"Skip {path}")
        return

    if sysargs.get_arg("backup"):
        from pathlib import Path
        import shutil
        p = Path(path)
        backup = p.with_name(p.name + ".old")
        shutil.copy(p, backup)

    img = Image.open(path).convert("RGBA")
    result = func(sysargs, img, path)
    if not path.lower().endswith(".png"):
        result = result.convert("RGB")
    result.save(path)