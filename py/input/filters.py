from pathspec import PathSpec
#ROOT = Path(sys.executable).parent if getattr(sys, "frozen", False) else Path.cwd()

filter_type = None
filter_list = None

def filter_init(sysargs):
    global filter_type
    global filter_list

    filters = sysargs.filters
    for _type in filters:
        filter_type = _type
        filter_list = PathSpec.from_lines("gitwildmatch", filters[_type])
        break

# noinspection PyUnresolvedReferences
def skip_file(path):
    global filter_type
    global filter_list

    if filter_type is not None:
        return filter_list.match_file(path)

    return False