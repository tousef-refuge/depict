import json
import sys

# noinspection PyUnresolvedReferences
class _ArgBase:
    def __getitem__(self, idx):
        return self._data[idx]

class SysArgs(_ArgBase):
    def __init__(self):
        self._data = json.loads(sys.argv[1])
        self.filters = json.loads(sys.argv[2])

class SubArgs(_ArgBase):
    def __init__(self, sysargs, path):
        self._data = sysargs._data
        self._data["path"] = path
