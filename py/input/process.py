from .filters import skip_file
from colorama import Fore
from PIL import Image
from py import image_output
import cv2
import numpy as np
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
    cap = cv2.VideoCapture(path)

    fps = cap.get(cv2.CAP_PROP_FPS)
    width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
    height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

    fourcc = cv2.VideoWriter_fourcc(*"mp4v")
    out = cv2.VideoWriter(path, fourcc, fps, (width, height))

    frame_number = 1
    while True:
        ret, frame = cap.read()
        if not ret:
            break

        img = Image.fromarray(cv2.cvtColor(frame, cv2.COLOR_BGR2RGB))
        img = func(sysargs, img, f"{path} (Frame {frame_number})")
        frame_number += 1

        frame = cv2.cvtColor(
            np.array(img.convert("RGB")),
            cv2.COLOR_RGB2BGR
        )
        out.write(frame)

def _image(func, sysargs, path):
    img = Image.open(path).convert("RGBA")
    result = func(sysargs, img, path)
    if not path.lower().endswith(".png"):
        result = result.convert("RGB")
    result.save(path)