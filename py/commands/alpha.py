from PIL import ImageEnhance
from py import image_output

def alpha(subargs, img, path):
    img = img.copy()
    newalpha = float(subargs["alpha"])

    _alpha = img.getchannel('A')
    _alpha = ImageEnhance.Brightness(_alpha).enhance(newalpha)

    img.putalpha(_alpha)
    image_output(f"Adjusted opacity (x{newalpha}): ", path)
    return img