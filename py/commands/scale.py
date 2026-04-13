from PIL import Image
from py import image_output

def scale(subargs):
    file = subargs["path"]
    factor = float(subargs["scale"])
    img = Image.open(file).convert("RGBA")

    w, h = img.size
    scaled = img.resize((
        int(w * factor),
        int(h * factor)
    ))
    scaled.save(file)
    image_output(f"Scaled (x{factor}): ", file)