from PIL import Image, ImageOps
from py import image_output

def invert(subargs):
    file = subargs["path"]
    img = Image.open(file).convert("RGBA")

    r, g, b, a = img.split()
    rgb_inverted = ImageOps.invert(Image.merge("RGB", (r, g, b)))

    inverted = Image.merge("RGBA", (*rgb_inverted.split(), a))
    inverted.save(file)
    image_output(f"Inverted colors: ", file)