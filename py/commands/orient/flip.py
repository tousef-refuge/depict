from PIL import ImageOps
from py import image_output, print_error

def flip(subargs, img, path):
    img = img.copy()
    axis = subargs["axis"]
    if axis not in ('x', 'y'):
        print_error("Invalid axis. Must be x or y")

    flipped = (ImageOps.mirror if axis == 'x' else ImageOps.flip)(img)
    image_output(f"Flipped ({axis}-axis): ", path)
    return flipped