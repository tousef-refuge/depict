from colorama import Fore
from pathlib import Path

from py.input import filters
from py.printhelp import image_output

from .gif import process as gif
from .image import process as image
from .video import process as video

import os
import shutil

def process(func, sysargs, path):
    if filters.skip_file(path, func):
        image_output("Skipped: ", path, Fore.RED)
        return

    if sysargs.get_arg("backup"):
        p = Path(path)
        backup = p.with_name(p.name + ".old")
        shutil.copy(p, backup)

    ext = os.path.splitext(path)[1][1:]

    if ext == "mp4":
        final_func = video
    elif ext == "gif":
        final_func = gif
    else:
        final_func = image
    final_func(func, sysargs, path)