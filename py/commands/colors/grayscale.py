from PIL import Image
from py import image_output

def grayscale(_subargs, img, path):
    img = img.copy()

    r, g, b, a = img.split()
    gray = Image.merge("RGB", (r, g, b)).convert("L")
    rgb_gray = Image.merge("RGB", (gray, gray, gray))

    grayscaled = Image.merge("RGBA", (*rgb_gray.split(), a))
    image_output(f"Converted to grayscale: ", path)
    return grayscaled