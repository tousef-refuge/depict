from PIL import Image
from py import image_output

def resize(subargs):
    file = subargs.path
    width = int(subargs[1])
    height = int(subargs[2])
    img = Image.open(file).convert("RGBA")

    resized = img.resize((width, height))
    resized.save(file)
    image_output(f"Resized ({width} x {height}): ", file)