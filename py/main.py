from .commands import *
from .input import *
import colorama

def main():
    colorama.init()
    argv = SysArgs()
    match argv["name"]:
        case "trim" | "flip" | "scale" | "resize" | "alpha":
            command = {
                "trim" : trim,
                "flip" : flip,
                "scale" : scale,
                "resize" : resize,
                "alpha" : alpha
            }[argv["name"]]
            process_path(command, argv)

        case _:
            name_error = f"Invalid function name ({argv.name})"
            raise NameError(name_error)