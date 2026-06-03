from ._helper import set_hsv
from py import image_output

def hue(subargs, img, path):
    img = img.copy()
    val = subargs["value"]

    adjusted = set_hsv(img, h=val)
    image_output(f"Adjust hue ({val}): ", path)
    return adjusted