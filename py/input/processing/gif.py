from PIL import Image, ImageSequence

def process(func, sysargs, path):
    gif = Image.open(path)

    frames = []
    frame_number = 1
    for frame in ImageSequence.Iterator(gif):
        img = frame.convert("RGBA")
        img = func(sysargs, img, f"{path} (Frame {frame_number})")
        frame_number += 1
        frames.append(img.convert("RGB"))

    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        loop=0,
        duration=gif.info.get("duration")
    )