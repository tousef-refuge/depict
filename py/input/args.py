import json
import sys

class SysArgs:
    def __init__(self):
        self._data = json.loads(sys.argv[1])
        self.file_args = json.loads(sys.argv[2])

    def __getitem__(self, idx):
        return self._data[idx]

    def get_arg(self, name):
        return self.file_args[name]
