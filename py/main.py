from .commands import *
from .input import *

def main():
    argv = SysArgs()
    match argv.name:
        case "trim":
            process_path(trim, argv)