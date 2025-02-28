import random
from save import save_minion, load_minions
from utils import print_header
from data import SHADOW_RANKS

def attempt_arise(player, monster):
    if monster.is_defeated():
        print_header(f"Arise Attempt on {monster.name}")
        
        attempts = 3
        while attempts > 0:
            choice = input("Do you want to use Arise? (y/n): ")
            
            if choice.lower() == 'y':
                if random.randint(1, 2) == 1: # 50% chance to succeed
                    rank = SHADOW_RANKS[min(len(SHADOW_RANKS)-1, monster.power // 2)]
                    
                    # Save the monster as a shadow minion
                    save_minion(monster.name, rank, monster.power)
                    
                    # Reload the player's available minions
                    player.minions = load_minions()
                    
                    print_header(f"{monster.name} has become a shadow minion of rank {rank}!")
                    return
                else:
                    attempts -= 1
                    print(f">> {monster.name} resisted Arise! Attempts left: {attempts}\n")
            else:
                print(">> Arise attempt skipped.\n")
                return
        print(f">> {monster.name} resisted all attempts!\n")
