from .commands import *
import sys

def main():
    args = sys.argv
    name = args[1]
    match name:
        case "trim":
            trim(args[1:])