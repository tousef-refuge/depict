from py import image_output

#ANOTHER filler function
def compress(_subargs, img, path):
    img = img.copy()
    image_output("Compressed: ", path)
    return img