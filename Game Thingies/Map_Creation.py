class map:
    #Goal is to create a map for visualization
    def __init__(self, Length, Height):
        self.length = Length
        self.height = Height


#Ideas for 


def map_gen(length, height):
    mapInfo = []
    ActiveLayer = 0
    while len(mapInfo) < length:
        mapInfo.append("X")
        if (ActiveLayer < height and len(mapInfo) == length ):
            print(mapInfo)
            mapInfo = []
            ActiveLayer += 1

area = map(10, 10)
map_gen(area.length, area.height)
