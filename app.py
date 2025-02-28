import random
import time

from save import load_minions, save_minion

# Shadow ranks based on Solo Leveling
SHADOW_RANKS = ["Soldier", "Elite", "Knight",
                "Elite Knight", "Commander", "Marshal"]
GATE_RANKS = ["E-Rank", "D-Rank", "C-Rank", "B-Rank", "A-Rank", "S-Rank"]
ABILITIES = {
    "Dagger Slash": 15,
    "Stealth Attack": 25,
    "Quicksilver": 30,
    "Shadow Minions Attack": 0,  # Damage calculated based on minions
}

MONSTER_LIST = [
    {"name": "Goblin", "power": 2},
    {"name": "Hobgoblin", "power": 3},
    {"name": "Orc", "power": 4},
    {"name": "Ogre", "power": 5},
    {"name": "Lizardman", "power": 6},
    {"name": "Minotaur", "power": 7},
    {"name": "Giant", "power": 8},
    {"name": "Demon", "power": 9},
    {"name": "Dragon", "power": 10}
]


def print_header(text):
    print("\n" + "=" * 50)
    print(text.center(50))
    print("=" * 50 + "\n")


class Monster:
    def __init__(self, name, power):
        self.name = name
        self.hp = power * 10
        self.power = power

    def is_defeated(self):
        return self.hp <= 0


class SungJinwoo:
    def __init__(self):
        self.hp = 100
        self.minions = load_minions()

    def attack(self, monster, ability=None):
        if ability:
            if ability == "Shadow Minions Attack" and self.minions:
                damage = sum(minion['power'] * 5 for minion in self.minions)
            else:
                damage = ABILITIES.get(ability, 10)
        else:
            damage = random.randint(10, 20)

        monster.hp -= damage
        print(
            f"\n>> You used {ability if ability else 'Basic Attack'}! {monster.name} took {damage} damage. Remaining HP: {monster.hp}\n")

    def attempt_arise(self, monster):
        if monster.is_defeated():
            print_header(f"Arise Attempt on {monster.name}")
            attempts = 3
            while attempts > 0:
                choice = input("Do you want to use Arise? (y/n): ")
                if choice.lower() == 'y':
                    if random.randint(1, 2) == 1:
                        rank = SHADOW_RANKS[min(
                            len(SHADOW_RANKS)-1, monster.power // 2)]
                        save_minion(monster.name, rank, monster.power)

                        print(
                            f"\n>> {monster.name} has become a shadow minion of rank {rank}!\n")
                        return
                    else:
                        attempts -= 1
                        print(
                            f">> {monster.name} resisted Arise! Attempts left: {attempts}\n")
                else:
                    print(">> Arise attempt skipped.\n")
                    return
            print(f">> {monster.name} resisted all attempts!\n")

    def is_defeated(self):
        return self.hp <= 0

    def show_minions(self):
        print_header("Your Shadow Minions")
        if not self.minions:
            print("You have no shadow minions.")
        else:
            for minion in self.minions:
                print(
                    f"- {minion['name']} (Rank: {minion['rank']}, Power: {minion['power']})")
        print("\n")


def generate_gate():
    rank_index = random.randint(0, len(GATE_RANKS) - 1)
    rank = GATE_RANKS[rank_index]
    monsters = [
        Monster(monster_data["name"], monster_data["power"])
        for monster_data in random.sample(MONSTER_LIST, random.randint(1, 3))
    ]
    return rank, monsters


def play_game():
    player = SungJinwoo()
    print_header("Welcome, Shadow Monarch! Prepare for battle.")

    while not player.is_defeated():
        command = input(
            "Enter 'g' to find a gate, 'm' to view minions, or 'q' to quit: ")
        if command == 'q':
            break
        elif command == 'm':
            player.show_minions()
            continue

        gate_rank, monsters = generate_gate()
        print_header(f"You entered a {gate_rank} Gate!")
        print(f"{len(monsters)} monsters appear!\n")

        for monster in monsters:
            print_header(f"A {monster.name} (Power: {monster.power}) appears!")
            while not monster.is_defeated() and not player.is_defeated():
                action = input("Attack (a) | Ability (b): ")
                if action == 'a':
                    player.attack(monster)
                elif action == 'b':
                    ability = input(
                        f"Choose ability: {list(ABILITIES.keys())}: ")
                    if ability in ABILITIES:
                        player.attack(monster, ability)
                else:
                    print("Invalid action.")

                if not monster.is_defeated():
                    damage = monster.power * 5
                    player.hp -= damage
                    print(
                        f"\n>> {monster.name} attacks! You took {damage} damage. Your HP: {player.hp}\n")

                if player.is_defeated():
                    print_header("You have fallen in battle!")
                    return

            player.attempt_arise(monster)

    print_header("Game Over! Your shadow army:")
    player.show_minions()


if __name__ == "__main__":
    play_game()
