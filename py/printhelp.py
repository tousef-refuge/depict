from colorama import Fore

def image_output(action, path, color=Fore.BLUE):
    if path is not None:
        print(color + action, end='')
        print(Fore.RESET + path)

def print_error(message):
    print(Fore.RED + message)
    print(Fore.RESET, end=' ')

def frame_num(frame):
    return f"{Fore.GREEN + f"(Frame {frame})"}{Fore.RESET + ' '}"