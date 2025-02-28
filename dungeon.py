import random
from data import DUNGEONS, MONSTER_LIST
from monster import Monster


def generate_gate():
    # Select a dungeon based on spawn rates
    selected_dungeon = random.choices(DUNGEONS, weights=[dungeon['spawn_rate'] for dungeon in DUNGEONS], k=1)

    # selected_dungeon will return a list, so you can access the first item
    selected_dungeon = selected_dungeon[0]
    
    # Extract dungeon name and rank of spawned dungeon
    dungeon_name = selected_dungeon['name']
    dungeon_rank = selected_dungeon['rank']
    
    # Filter monsters that can spawn in this dungeon rank
    valid_monsters = [m for m in MONSTER_LIST if dungeon_rank in m["dungeon_ranks"]]
    
    # Edge case: No valid monsters available
    if not valid_monsters:
        print("No valid monsters available for this dungeon!")
        return dungeon_name, []

    # Spawn 1-3 random monsters from the valid list
    num_monsters = random.randint(1, 3)
    monsters = random.choices(valid_monsters, k=num_monsters)

    return dungeon_name, [Monster(m["name"], m["power"]) for m in monsters]
