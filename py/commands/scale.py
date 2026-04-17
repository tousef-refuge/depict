from py import image_output

def scale(subargs, img, path):
    img = img.copy()
    factor = float(subargs["scale"])

    w, h = img.size
    scaled = img.resize((
        int(w * factor),
        int(h * factor)
    ))
    image_output(f"Scaled (x{factor}): ", path)
    return scaled