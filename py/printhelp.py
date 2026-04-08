from colorama import Fore

def image_output(action, path):
    print(Fore.BLUE + action, end='')
    print(Fore.RESET + path)

def print_error(message):
    print(Fore.RED + message)
    print(Fore.RESET)