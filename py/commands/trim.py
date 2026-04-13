from PIL import Image
from py import image_output

def trim(subargs):
    file = subargs["path"]
    img = Image.open(file).convert("RGBA")

    bbox = img.split()[3].getbbox()
    trimmed = img.crop(bbox)
    trimmed.save(file)
    image_output("Trimmed: ", file)
