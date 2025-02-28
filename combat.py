import random
from data import ABILITIES

def attack(player, monster, ability=None):
    if ability:
        
        # Special Skill for Shadow Minions that 
        # multiplies the total power of all minions by 5
        if ability == "Shadow Minions Attack" and player.minions:
            damage = sum(minion['power'] * 5 for minion in player.minions)
            
        # If the ability is not Shadow Minions Attack,
        # get the damage from the ABILITIES dictionary
        # or default to 10 if the ability is not found
        else:
            damage = ABILITIES.get(ability, {"damage": 10})['damage']
    
    # If no ability is chosen (basic attack), deal random damage between 10 and 20
    else:
        damage = random.randint(10, 20)

    # Deal the damage to the monster
    monster.hp -= damage
    print(f"\n>> You used {ability if ability else 'Basic Attack'}! \n {monster.name} took {damage} damage. \n {monster.name} Remaining HP: {monster.hp}\n")
