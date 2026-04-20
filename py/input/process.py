from .filters import skip_file
from colorama import Fore
from PIL import Image
from py import image_output
import os

def process(func, sysargs, path):
    if skip_file(path, func):
        image_output("Skipped: ", path, Fore.RED)
        return

    if sysargs.get_arg("backup"):
        from pathlib import Path
        import shutil
        p = Path(path)
        backup = p.with_name(p.name + ".old")
        shutil.copy(p, backup)

    ext = os.path.splitext(path)[1][1:]
    if ext == "mp4":
        _video(func, sysargs, path)
    else:
        _image(func, sysargs, path)

def _video(func, sysargs, path):
    pass

def _image(func, sysargs, path):
    img = Image.open(path).convert("RGBA")
    result = func(sysargs, img, path)
    if not path.lower().endswith(".png"):
        result = result.convert("RGB")
    result.save(path)