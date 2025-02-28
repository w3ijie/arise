from player import SungJinwoo
from dungeon import generate_gate
from utils import print_header, get_user_command, print_creepy_pause

def play_game():
    player = SungJinwoo()
    print_header("Welcome, Shadow Monarch! Prepare for battle.")

    # Main game loop - gameover when player dies or quits
    while not player.is_defeated():
                
        # Get user command
        command = get_user_command()
        if command == '3':
            break
        elif command == '2':
            player.show_minions()
        elif command == '1':
            pass
        
        # Generate a dungeon gate
        dungeon_name, monsters = generate_gate()
        
        print_header(f"You encounter a {dungeon_name}!")

        # Choice between entering the gate or not
        choice = input("\nDo you want to enter the gate? (y/n): ")
        if choice.lower() != 'y':
            print(f"{len(monsters)} monsters detected...\n")
            continue
        
        # Fight monsters in the dungeon
        for monster in monsters:
            player.fight_monster(monster)

    print_header("See you again ShAdOw MoNaRcH~")
    print_creepy_pause()
    print("The shadows stir, your Shadow Army silently awaits your return from the abyss.")
    print_creepy_pause()
    player.show_minions()

if __name__ == "__main__":
    play_game()
