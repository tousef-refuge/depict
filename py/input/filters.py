filter_type = None
filter_list = None

def filter_init(sysargs):
    global filter_type
    global filter_list

    filters = sysargs.filters
    for _type in filters:
        filter_type = _type
        filter_list = filters[_type]
        break

def skip_file(path):
    global filter_type
    global filter_list

    if filter_type is not None:
        if path in filter_list:
            return True

    return False