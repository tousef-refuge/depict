from .commands import *
from .input import *
import colorama

COMMANDS = {
    "trim" : trim,
    "flip" : flip,
    "scale" : scale,
    "resize" : resize,
    "alpha" : alpha,
    "invert" : invert,
    "grayscale" : grayscale,
    "backup" : backup
}

def main():
    colorama.init()
    argv = SysArgs()

    name = argv["name"]
    if name not in COMMANDS.keys():
        name_error = f"Invalid function name ({argv["name"]})"
        raise NameError(name_error)

    command = COMMANDS[name]
    if name == "backup":
        argv.file_args["backup"] = True
    process_path(command, argv)