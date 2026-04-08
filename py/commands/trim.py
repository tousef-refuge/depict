from PIL import Image

def trim(subargs):
    file = subargs.path
    img = Image.open(file).convert("RGBA")

    bbox = img.split()[3].getbbox()
    trimmed = img.crop(bbox)
    trimmed.save(file)
    print(f"Cropped: {file}")
