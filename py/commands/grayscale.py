from PIL import Image, ImageOps
from py import image_output

def grayscale(subargs):
    file = subargs["path"]
    img = Image.open(file).convert("RGBA")

    r, g, b, a = img.split()
    gray = Image.merge("RGB", (r, g, b)).convert("L")
    rgb_gray = Image.merge("RGB", (gray, gray, gray))

    grayscaled = Image.merge("RGBA", (*rgb_gray.split(), a))
    grayscaled.save(file)
    image_output(f"Converted to grayscale: ", file)