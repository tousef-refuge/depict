from .args import SubArgs
from py import print_error
import os

filter_type = None
filter_list = None

def process_path(func, sysargs):
    global filter_type
    global filter_list

    filters = sysargs.filters
    for _type in filters:
        filter_type = _type
        filter_list = filters[_type]
        break

    root = sysargs["path"]
    if os.path.isfile(root):
        func(SubArgs(sysargs, root))
    elif os.path.isdir(root):
        _dir_walk(func, sysargs)
    else:
        print_error("Path does not exist")

def _dir_walk(func, sysargs):
    root = sysargs["path"]
    for current_path, dirs, files in os.walk(root):
        for file in files:
            if file.lower().endswith(".png"):
                full_path = os.path.join(current_path, file)
                if _skip_file(full_path):
                    continue

                try:
                    func(SubArgs(sysargs, full_path))
                except Exception as e:
                    print(f"Error processing {full_path}: {e}")

def _skip_file(path):
    global filter_type
    global filter_list

    if filter_type is not None:
        if path in filter_list:
            return True

    return False