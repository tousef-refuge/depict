from PIL import Image
from py import frame_num, get_config
import cv2
import numpy as np
import os
import tempfile

def process(func, sysargs, path):
    print_frames = get_config("print_frames")
    cap = cv2.VideoCapture(path)

    # noinspection PyUnresolvedReferences
    fourcc = cv2.VideoWriter_fourcc(*"mp4v")
    dir_name = os.path.dirname(path)
    with tempfile.NamedTemporaryFile(delete=False, suffix=".mp4", dir=dir_name) as tmp:
        temp = tmp.name
    out = None

    frame_number = 1
    sample_img = None
    while True:
        ret, frame = cap.read()
        if not ret:
            break

        img = Image.fromarray(cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)).convert("RGBA")
        img = func(sysargs, img, f"{path} {frame_num(frame_number)}" if print_frames else None)
        if frame_number == 1:
            sample_img = img
        frame_number += 1

        frame = cv2.cvtColor(
            np.array(img),
            cv2.COLOR_RGB2BGR
        )

        if out is None:
            fps = cap.get(cv2.CAP_PROP_FPS)
            out = cv2.VideoWriter(temp, fourcc, fps, img.size)
        out.write(frame)
    if not print_frames:
        func(sysargs, sample_img, path)

    cap.release()
    out.release()
    os.replace(temp, path)