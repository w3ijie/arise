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

# Monsters list with allowed dungeon ranks
MONSTER_LIST = [
    {"name": "Goblin", "power": 2, "dungeon_ranks": ["E"]},
    {"name": "Hobgoblin", "power": 3, "dungeon_ranks": ["E", "D"]},
    {"name": "Orc", "power": 4, "dungeon_ranks": ["D", "C"]},
    {"name": "Ogre", "power": 5, "dungeon_ranks": ["D", "C"]},
    {"name": "Lizardman", "power": 6, "dungeon_ranks": ["C", "B"]},
    {"name": "Minotaur", "power": 7, "dungeon_ranks": ["B"]},
    {"name": "Giant", "power": 8, "dungeon_ranks": ["B", "A"]},
    {"name": "Demon", "power": 9, "dungeon_ranks": ["A", "S"]},
    {"name": "Dragon", "power": 10, "dungeon_ranks": ["S"]}
]

# Dungeons list with spawn rates and rank tagging
DUNGEONS = [
    {"name": "E-Rank Dungeon", "spawn_rate": 30, "rank": "E"},
    {"name": "D-Rank Dungeon", "spawn_rate": 25, "rank": "D"},
    {"name": "C-Rank Dungeon", "spawn_rate": 20, "rank": "C"},
    {"name": "B-Rank Dungeon", "spawn_rate": 15, "rank": "B"},
    {"name": "A-Rank Dungeon", "spawn_rate": 7, "rank": "A"},
    {"name": "S-Rank Dungeon", "spawn_rate": 3, "rank": "S"}
]