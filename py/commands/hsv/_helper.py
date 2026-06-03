#circular imports my BIGGEST OPP
from PIL import Image

def set_hsv(img, h=None, s=None, v=None):
    alpha = None
    if img.mode == "RGBA":
        alpha = img.getchannel("A")

    hsv = img.convert("HSV")
    nh, ns, nv = hsv.split()

    if h is not None:
        nh = nh.point(lambda _: h)
    if s is not None:
        ns = ns.point(lambda _: s)
    if v is not None:
        nv = nv.point(lambda _: v)

    result = Image.merge("HSV", (nh, ns, nv)).convert("RGB")
    if alpha is not None:
        result.putalpha(alpha)
    return result