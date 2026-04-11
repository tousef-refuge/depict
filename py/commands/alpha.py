from PIL import Image, ImageEnhance
from py import image_output

def alpha(subargs):
    file = subargs.path
    newalpha = float(subargs[1])
    img = Image.open(file).convert("RGBA")

    _alpha = img.getchannel('A')
    _alpha = ImageEnhance.Brightness(_alpha).enhance(newalpha)

    img.putalpha(_alpha)
    img.save(file)
    image_output(f"Adjusted opacity (x{newalpha}): ", file)