from py import image_output, print_error

def rotate(subargs, img, path):
    img = img.copy()
    direction = subargs["direction"]
    if direction not in ('l', 'r'):
        print_error("Invalid axis. Must be l or r")

    if direction == 'l':
        text = "left"
        degrees = 90
    else:
        text = "right"
        degrees = -90
    rotated = img.rotate(degrees)
    image_output(f"Rotated (90 degrees {text}): ", path)
    return rotated