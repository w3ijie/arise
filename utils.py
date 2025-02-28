import random
import bisect
import time

# Print a header with a centered text
def print_header(text):
    print("\n" + "=" * 50)
    print(text.center(50))
    print("=" * 50 )

# Print the main menu options
def show_main_menu():
    print("\n--- Main Menu ---")
    print("1. Find a Dungeon")
    print("2. View Minions")
    print("3. Quit")
    print("-----------------\n")

# Get the user's choice from the main menu
def get_user_command():
    while True:
        show_main_menu()
        command = input("Please enter your choice: ").strip().lower()

        if command == '1':
            return '1'
        elif command == '2':
            return '2'
        elif command == '3':
            return '3'
        else:
            print("Invalid input! Please enter '1', '2', or '3'.")
            
# Print a ... pausing effect
def print_creepy_pause():
    print()
    print(end="")
    for _ in range(3):  # Print three dots with a small delay
        time.sleep(0.5)
        print(".", end="", flush=True)
    time.sleep(0.5)
    print("\n")  # Move to the next line after the dots
