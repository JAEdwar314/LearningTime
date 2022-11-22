
#Creating Player Tracking

class Player:
    #Important things for the player for now // Name, Health, Coordiantes
    def __init__ (self, Name, Health, Coord):
        self.name = Name
        self.health = Health
        self.coord = Coord

test = Player("Jack", "100", [0,0])
