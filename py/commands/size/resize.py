from py import image_output

def resize(subargs, img, path):
    img = img.copy()
    width = int(subargs["width"])
    height = int(subargs["height"])

    resized = img.resize((width, height))
    image_output(f"Resized ({width} x {height}): ", path)
    return resized