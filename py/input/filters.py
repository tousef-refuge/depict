from collections import defaultdict
from pathspec import PathSpec
import os
#ROOT = Path(sys.executable).parent if getattr(sys, "frozen", False) else Path.cwd()

filter_type = None
filter_list = PathSpec.from_lines("gitwildmatch", {}) #mo more pyinspection hell

VALID_FILTER_TYPES = ("ignore", "only")
EXT_FUNC_DICT = defaultdict(list)
ALL_FUNCS = ["alpha", "backup", "flip", "grayscale", "invert", "resize", "scale", "trim"]

def filter_init(sysargs):
    global filter_type
    global filter_list

    file_args = sysargs.file_args
    for _type in file_args:
        if _type not in VALID_FILTER_TYPES:
            continue
        filter_type = _type
        filter_list = PathSpec.from_lines("gitwildmatch", file_args[_type])
        break

    valid_exts = "png", "jpg", "jpeg", "mp4"
    for ext in valid_exts:
        EXT_FUNC_DICT[ext].extend(ALL_FUNCS)
    EXT_FUNC_DICT["mp4"].remove("trim")

def skip_file(path, func):
    #first step : input flags
    global filter_type
    global filter_list

    if filter_type is not None:
        if filter_type == "ignore":
            return filter_list.match_file(path)

        if filter_type == "only":
            return not filter_list.match_file(path)

    #second step : match extensions
    ext = os.path.splitext(path)[1][1:]
    return func.__name__ not in EXT_FUNC_DICT[ext]