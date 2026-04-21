# noinspection PyUnusedImports
from .commands import * #save the sustainability society GLOBALS()
from .input import filters, process_path, SysArgs
from py import commands
import colorama

def main():
    colorama.init()
    argv = SysArgs()

    name = argv["name"]
    if name not in commands.__all__:
        raise NameError(f"Invalid function name ({argv["name"]})")

    command = globals()[name]
    argv.file_args["backup"] |= name == "backup"

    filters.init(argv)
    process_path(command, argv)