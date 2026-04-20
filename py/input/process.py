from .filters import skip_file
from colorama import Fore
from PIL import Image
from py import image_output

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

    img = Image.open(path).convert("RGBA")
    result = func(sysargs, img, path)
    if not path.lower().endswith(".png"):
        result = result.convert("RGB")
    result.save(path)