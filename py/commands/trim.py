from py import image_output

def trim(_subargs, img, path):
    img = img.copy()

    bbox = img.split()[3].getbbox()
    trimmed = img.crop(bbox)
    image_output("Trimmed: ", path)
    return trimmed
