from PIL import Image, ImageOps
from py import image_output, print_error

def flip(subargs):
    file = subargs["path"]
    axis = subargs["axis"]
    if axis not in ('x', 'y'):
        print_error("Invalid axis. Must be x or y")

    img = Image.open(file).convert("RGBA")

    flipped = (ImageOps.mirror if axis == 'x' else ImageOps.flip)(img)
    flipped.save(file)
    image_output(f"Flipped ({axis}-axis): ", file)