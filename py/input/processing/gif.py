from PIL import Image, ImageSequence
from py import frame_num, get_config

def process(func, sysargs, path):
    print_frames = get_config("print_frames")
    gif = Image.open(path)

    frames = []
    frame_number = 1
    sample_img = None
    for frame in ImageSequence.Iterator(gif):
        img = frame.convert("RGBA")
        img = func(sysargs, img, f"{path} {frame_num(frame_number)}" if print_frames else None)
        if frame_number == 1:
            sample_img = img
        frame_number += 1
        frames.append(img.convert("RGB"))
    if not print_frames:
        func(sysargs, sample_img, path)

    frames[0].save(
        path,
        optimize=sysargs.get_arg("compress"),
        save_all=True,
        append_images=frames[1:],
        loop=0,
        duration=gif.info.get("duration")
    )