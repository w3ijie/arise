import random
from save import load_minions, load_player_level
from combat import attack
from arise import attempt_arise
from utils import print_header
from data import ABILITIES

class SungJinwoo:
    def __init__(self):
        self.hp = 100
        self.level = load_player_level()
        self.minions = load_minions()

    # Method to attack a monster
    def fight_monster(self, monster):
        print_header(f"A {monster.name} (Power: {monster.power}) appears!")
        
        while not monster.is_defeated() and not self.is_defeated():
            self.choose_ability(monster)

            if not monster.is_defeated():
                self.take_damage(monster)

        attempt_arise(self, monster)

    # Method to choose an ability to attack a monster
    def choose_ability(self, monster):
        print("Your Abilities:")
        for i, ability in enumerate(ABILITIES.keys(), 1):
            print(f"{i}. {ability}")
        action = input("\nChoose ability number: ")

        if action.isdigit() and 1 <= int(action) <= len(ABILITIES):
            ability = list(ABILITIES.keys())[int(action) - 1]
            attack(self, monster, ability)
        elif action == 'a':
            attack(self, monster)
        else:
            print("Invalid action.")

    # Method when player takes damage from a monster
    def take_damage(self, monster):
        damage = monster.power * 5
        self.hp -= damage
        
        print(f"\n>> {monster.name} attacks! \n You took {damage} damage. \n Your HP: {self.hp}\n")

    # Method to check if player is defeated
    def is_defeated(self):
        if self.hp <= 0:
            print_header("You have fallen in battle... The shadows claim your soul.")
        return self.hp <= 0

    # Method to show the player's shadow minions
    def show_minions(self):
        print_header("Your Shadow Minions")
        if not self.minions:
            print("You have no shadow minions.")
        else:
            for minion in self.minions:
                print(f"- {minion['name']} (Rank: {minion['rank']}, Power: {minion['power']})")
        print("\n")
