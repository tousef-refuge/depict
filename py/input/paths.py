import os

def process_path(func, path):
    if os.path.isfile(path):
        func(path)
    elif os.path.isdir(path):
        _dir_walk(func, path)
    else:
        print("Path does not exist")

def _dir_walk(func, root_dir):
    for current_path, dirs, files in os.walk(root_dir):
        for file in files:
            if file.lower().endswith(".png"):
                full_path = os.path.join(current_path, file)
                try:
                    func(full_path)
                except Exception as e:
                    print(f"Error processing {full_path}: {e}")