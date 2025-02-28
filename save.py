import datetime
import json
from typing import List


def save_minion(name, rank, power):
    minion_data = {
        "name": name,
        "rank": rank,
        "power": power,
        "date_of_arise": datetime.datetime.now().isoformat()
    }

    try:
        with open('save_state.json', 'r+') as file:
            data = json.load(file)
            data['minions'].append(minion_data)
            file.seek(0)
            json.dump(data, file, indent=4)
    except FileNotFoundError:
        with open('save_state.json', 'w') as file:
            json.dump({"minions": [minion_data]}, file, indent=4)


def load_minions() -> List[dict]:
    try:
        with open('save_state.json', 'r') as file:
            data = json.load(file)
            return data.get('minions', [])
    except FileNotFoundError:
        return []


def load_player_level() -> int:
    try:
        with open('save_state.json', 'r') as file:
            data = json.load(file)
            player = data.get('player', {})
            return player.get('level')
    except FileNotFoundError:
        return []
