class Monster:
    def __init__(self, name, power):
        self.name = name
        self.hp = power * 10
        self.power = power

    def is_defeated(self):
        return self.hp <= 0
