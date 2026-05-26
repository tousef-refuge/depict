from PIL import Image, ImageOps
from py import image_output

def invert(_subargs, img, path):
    img = img.copy()

    r, g, b, a = img.split()
    rgb_inverted = ImageOps.invert(Image.merge("RGB", (r, g, b)))

    inverted = Image.merge("RGBA", (*rgb_inverted.split(), a))
    image_output(f"Inverted colors: ", path)
    return inverted