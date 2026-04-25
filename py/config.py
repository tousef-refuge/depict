from pathlib import Path
import json
import sys

_DATA = {}
ROOT = Path(sys.executable).parent if getattr(sys, "frozen", False) else Path.cwd()

def config_init():
    global _DATA
    global ROOT
    with open(ROOT / "config.json") as file:
        _DATA = json.load(file)

def get_config(key):
    return _DATA[key]