from colorama import Fore

def image_output(action, path, color=Fore.BLUE):
    print(color + action, end='')
    print(Fore.RESET + path)

def print_error(message):
    print(Fore.RED + message)
    print(Fore.RESET, end=' ')