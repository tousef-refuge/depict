from .commands import *
from .input import *
import colorama

def main():
    colorama.init()
    argv = SysArgs()
    match argv.name:
        case "trim":
            process_path(trim, argv)

        case "flip":
            process_path(flip, argv)