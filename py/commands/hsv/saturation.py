from ._helper import set_hsv
from py import image_output

def saturation(subargs, img, path):
    img = img.copy()
    val = subargs["value"]

    adjusted = set_hsv(img, s=val)
    image_output(f"Adjust saturation ({val}): ", path)
    return adjusted