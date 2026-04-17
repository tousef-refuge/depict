from py import image_output

#literally just a filler function lmao
def backup(_subargs, img, path):
    img = img.copy()
    image_output("Created backup: ", path)
    return img