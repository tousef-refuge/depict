from .commands import *
from .input import *
import sys

def main():
    args = sys.argv
    name = args[1]
    match name:
        case "trim":
            process_path(trim, args[2])