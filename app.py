import random
import time

from save import load_minions, save_minion, load_player_level

# Shadow ranks based on Solo Leveling
SHADOW_RANKS = ["Soldier", "Elite", "Knight", "Elite Knight", "Commander", "Marshal"]
GATE_RANKS = ["E-Rank", "D-Rank", "C-Rank", "B-Rank", "A-Rank", "S-Rank"]

# Abilities dictionary
ABILITIES = {
    "Dagger Slash": {"damage": 15, "spawn_rate": 30},
    "Stealth Attack": {"damage": 25, "spawn_rate": 25},
    "Quicksilver": {"damage": 30, "spawn_rate": 20},
    "Shadow Minions Attack": {"damage": 0, "spawn_rate": 25}  # Damage calculated based on minions
}

# Monsters list with spawn rates
MONSTER_LIST = [
    {"name": "Goblin", "power": 2, "spawn_rate": 30},
    {"name": "Hobgoblin", "power": 3, "spawn_rate": 25},
    {"name": "Orc", "power": 4, "spawn_rate": 20},
    {"name": "Ogre", "power": 5, "spawn_rate": 15},
    {"name": "Lizardman", "power": 6, "spawn_rate": 10},
    {"name": "Minotaur", "power": 7, "spawn_rate": 7},
    {"name": "Giant", "power": 8, "spawn_rate": 5},
    {"name": "Demon", "power": 9, "spawn_rate": 3},
    {"name": "Dragon", "power": 10, "spawn_rate": 2}
]

# Dungeons list with spawn rates and monsters subset
DUNGEONS = [
    {"name": "E-Rank Dungeon", "spawn_rate": 30, "monsters": MONSTER_LIST[:3]},
    {"name": "D-Rank Dungeon", "spawn_rate": 25, "monsters": MONSTER_LIST[3:6]},
    {"name": "C-Rank Dungeon", "spawn_rate": 20, "monsters": MONSTER_LIST[6:8]},
    {"name": "B-Rank Dungeon", "spawn_rate": 15, "monsters": MONSTER_LIST[7:9]},
    {"name": "A-Rank Dungeon", "spawn_rate": 7, "monsters": MONSTER_LIST[8:]},
    {"name": "S-Rank Dungeon", "spawn_rate": 3, "monsters": MONSTER_LIST[9:]}
]


def print_header(text):
    print("\n" + "=" * 50)
    print(text.center(50))
    print("=" * 50 + "\n")


class Monster:
    def __init__(self, name, power, spawn_rate):
        self.name = name
        self.hp = power * 10
        self.power = power
        self.spawn_rate = spawn_rate

    def is_defeated(self):
        return self.hp <= 0


class SungJinwoo:
    def __init__(self):
        self.hp = 100
        self.level = load_player_level()
        self.minions = load_minions()

    def attack(self, monster, ability=None):
        if ability:
            if ability == "Shadow Minions Attack" and self.minions:
                damage = sum(minion['power'] * 5 for minion in self.minions)
            else:
                damage = ABILITIES.get(ability, {"damage": 10})['damage']
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
                        self.minions = load_minions()

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


def weighted_choice(options, key):
    total = sum(option[key] for option in options)
    pick = random.uniform(0, total)
    current = 0
    for option in options:
        current += option[key]
        if current > pick:
            return option


def generate_gate():
    # Select dungeon based on weighted spawn rate
    dungeon = weighted_choice(DUNGEONS, "spawn_rate")
    dungeon_name = dungeon['name']
    valid_monsters = dungeon["monsters"]

    # Ensure we don't sample more monsters than are available in the dungeon
    num_monsters = random.randint(1, min(3, len(valid_monsters)))
    monsters = random.sample(valid_monsters, num_monsters)

    return dungeon_name, [Monster(monster["name"], monster["power"], monster["spawn_rate"]) for monster in monsters]


def play_game():
    player = SungJinwoo()
    print_header("Welcome, Shadow Monarch! Prepare for battle.")

    while not player.is_defeated():
        command = input(
            "Enter 'g' to find a dungeon, 'm' to view minions, or 'q' to quit: ")
        if command == 'q':
            break
        elif command == 'm':
            player.show_minions()
            continue

        dungeon_name, monsters = generate_gate()
        print_header(f"You encounter a {dungeon_name}!")
        print(f"{len(monsters)} monsters detected!\n")

        # Player decision to enter or walk away
        choice = input("Do you want to enter the gate? (y/n): ")
        if choice.lower() != 'y':
            continue

        # Battle with the spawned monsters
        for monster in monsters:
            print_header(f"A {monster.name} (Power: {monster.power}) appears!")
            while not monster.is_defeated() and not player.is_defeated():
                print(f"Available abilities:")
                for i, ability in enumerate(ABILITIES.keys(), 1):
                    print(f"{i}. {ability}")
                action = input("Choose ability number: ")
                if action.isdigit() and 1 <= int(action) <= len(ABILITIES):
                    ability = list(ABILITIES.keys())[int(action) - 1]
                    player.attack(monster, ability)
                elif action == 'a':
                    player.attack(monster)
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
