from PIL import Image
import cv2
import numpy as np
import os
import tempfile

def process(func, sysargs, path):
    cap = cv2.VideoCapture(path)

    # noinspection PyUnresolvedReferences
    fourcc = cv2.VideoWriter_fourcc(*"mp4v")
    dir_name = os.path.dirname(path)
    with tempfile.NamedTemporaryFile(delete=False, suffix=".mp4", dir=dir_name) as tmp:
        temp = tmp.name
    out = None

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

        if out is None:
            fps = cap.get(cv2.CAP_PROP_FPS)
            out = cv2.VideoWriter(temp, fourcc, fps, img.size)
        out.write(frame)

    cap.release()
    out.release()
    os.replace(temp, path)