import sys

# noinspection PyUnresolvedReferences
class _ArgBase:
    def __getitem__(self, idx):
        return self._args[idx]

class SysArgs(_ArgBase):
    def __init__(self):
        self._args = sys.argv
        self.name = self._args[1]
        self.root = self._args[2]

class SubArgs(_ArgBase):
    def __init__(self, sysargs, path):
        self._args = sysargs[2:]
        self.path = path
