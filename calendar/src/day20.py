import time
import math
from collections import namedtuple
import re
import numpy as np

class Tile:

    def __init__(self, _id, image):
        self.id = _id
        self.image = image

    def __str__(self):
        return f'{self.id}'
def init():
    data = open('proto.txt', 'r').read().split("\n")
    fields = {}
    valid_values = set()
    your_ticket = []
    nearby_tickets = []
    group = 0
    tiles = []
    image = []
    for line in data:
        if "Tile" in line:
            _id = int(re.findall("\d+", line)[0])
            # print(_id)
        elif line == "":
            image = np.array(image)
            tiles.append(Tile(_id, image))
            image = []
        else:
            a = []
            for char in line:
                a.append(char)
            image.append(a)
        
            # print(image)


    def check(tile1, tile2):
        # print(tile1)
        for k in range(4):
            # print()
            tile1 = np.rot90(tile1)
            for i in range(4):

                tile2 = np.rot90(tile2)

                x = []
                y = []
                for j in range(10):
                    x.append(tile1[j][9])
                    y.append(tile2[j][9])
                # print(f"x: {''.join(x)}\ny: {''.join(y)}")
                if ''.join(x) == ''.join(y):
                    return (True, tile1)

                tile2 = np.flip(tile2, 0)
                x = []
                y = []
                for j in range(10):
                    x.append(tile1[j][9])
                    y.append(tile2[j][9])
                tile2 = np.flip(tile2, 0)
                if ''.join(x) == ''.join(y):
                    return (True, tile1)

                tile2 = np.flip(tile2, 1)
                x = []
                y = []
                
                for j in range(10):
                    x.append(tile1[j][9])
                    y.append(tile2[j][9])
                tile2 = np.flip(tile2, 1)
                if ''.join(x) == ''.join(y):
                    return (True, tile1)

                tile2 = np.flip(tile2, 0)
                tile2 = np.flip(tile2, 1)
                x = []
                y = []
                for j in range(10):
                    x.append(tile1[j][9])
                    y.append(tile2[j][9])
                tile2 = np.flip(tile2, 1)
                tile2 = np.flip(tile2, 0)
                if ''.join(x) == ''.join(y):
                    return (True, tile1)
        return (False, tile1)


            

    def check_right(tile1, tile2):
        for i in range(4):

            tile2 = np.rot90(tile2)

            x = []
            y = []
            for j in range(10):
                x.append(tile1[j][9])
                y.append(tile2[j][0])
            if ''.join(x) == ''.join(y):
                return (True, tile2)

            tile2 = np.flip(tile2, 0)
            x = []
            y = []
            for j in range(10):
                x.append(tile1[j][9])
                y.append(tile2[j][0])
            if ''.join(x) == ''.join(y):
                return (True, tile2)
            tile2 = np.flip(tile2, 0)
            

            tile2 = np.flip(tile2, 1)
            x = []
            y = []
            
            for j in range(10):
                x.append(tile1[j][9])
                y.append(tile2[j][0])
            
            if ''.join(x) == ''.join(y):
                return (True, tile2)
            tile2 = np.flip(tile2, 1)
            
            tile2 = np.flip(tile2, 0)
            tile2 = np.flip(tile2, 1)
            x = []
            y = []
            for j in range(10):
                x.append(tile1[j][9])
                y.append(tile2[j][0])
            
            if ''.join(x) == ''.join(y):
                return (True, tile2)

            tile2 = np.flip(tile2, 1)
            tile2 = np.flip(tile2, 0)
        return (False, tile2)

    def check_bottom(tile1, tile2):
        
        for i in range(4):

            tile2 = np.rot90(tile2)

            x = []
            y = []
            for j in range(10):
                x.append(tile1[9][j])
                y.append(tile2[0][j])
            if ''.join(x) == ''.join(y):
                return (True, tile2)

            tile2 = np.flip(tile2, 0)
            x = []
            y = []
            for j in range(10):
                x.append(tile1[9][j])
                y.append(tile2[0][j])
            if ''.join(x) == ''.join(y):
                return (True, tile2)
            tile2 = np.flip(tile2, 0)
            

            tile2 = np.flip(tile2, 1)
            x = []
            y = []
            
            for j in range(10):
                x.append(tile1[9][j])
                y.append(tile2[0][j])
            
            if ''.join(x) == ''.join(y):
                return (True, tile2)
            tile2 = np.flip(tile2, 1)
            
            tile2 = np.flip(tile2, 0)
            tile2 = np.flip(tile2, 1)
            x = []
            y = []
            for j in range(10):
                x.append(tile1[9][j])
                y.append(tile2[0][j])
            
            if ''.join(x) == ''.join(y):
                return (True, tile2)

            tile2 = np.flip(tile2, 1)
            tile2 = np.flip(tile2, 0)
        return (False, tile2)

    def strip_border(tile):
        new_tile = np.zeros((8,8), str)
        for i in range(1, 9):
            for j in range(1, 9):
                new_tile[j-1][i-1] = tile[j][i]
        return new_tile

    print(len(tiles))
    # Find corner piece
    for i in range(len(tiles)):
        c = 0
        
        for j in range(len(tiles)):
            if i == j:
                continue
            
        
            valid, temp = check(tiles[i].image, tiles[j].image)
            if valid:
                tiles[i].image = temp
                c+=1
        if c==2:
            print(f"corner piece {tiles[i].id}")
            corner = tiles[i]
            break
    grid = np.zeros((int(math.sqrt(len(tiles)))+1,int(math.sqrt(len(tiles)))+1), Tile)

    # this bit done manually sadly
    # aligning tile in top left corner so no match edges on border
    grid[0][0] = corner
    grid[0][0].image = np.rot90(grid[0][0].image)
    grid[0][0].image = np.rot90(grid[0][0].image)
    grid[0][0].image = np.flip(grid[0][0].image, 1)
    

    # build up grid
    for y in range(int(math.sqrt(len(tiles)))):
        for x in range(int(math.sqrt(len(tiles)))):
            for j in range(len(tiles)):
                if grid[y][x].id == tiles[j].id:
                    continue
                valid, temp = check_right(grid[y][x].image, tiles[j].image)
                # across
                if valid :# and tiles[j] not in used:
                    tiles[j].image = temp
                    grid[y][x+1] = tiles[j]
                # down
                valid, temp = check_bottom(grid[y][x].image, tiles[j].image)
                if valid:# and tiles[j] not in used:
                    tiles[j].image = temp
                    grid[y+1][x] = tiles[j]

    broken_map = np.zeros((len(tiles),len(tiles)), str)
    for i in range(int(math.sqrt(len(tiles)))):
        for j in range(int(math.sqrt(len(tiles)))):
            grid[i][j].image = strip_border(grid[i][j].image)
            for (y, row) in enumerate(grid[i][j].image):
                for (x, col) in enumerate(row):
                    broken_map[int(math.sqrt(len(tiles)))*i+y][int(math.sqrt(len(tiles)))*j+x] = col


    broken_map = [''.join(i) for i in broken_map]
    return broken_map
    
# init()



# Monster Hunting
seamap =['...#........#.......#.......#.................#..##....#.......##.........#...........#..#..#...',
 '#.....##...##....#.......#.....#.......###.......####...##..........##.#........#.........#.....',
 '........#.......##.....#.....#.#..#.#....#....#....#...#.#.#.............#.........#.#......#..#',
 '..#...#........#....................#....#..#.#...#.#.#........#...#..#.#...##......#..#.....#.#',
 '#.....#...#..#..#...#.#.#........#..###..........###.............#..#..#.#.....##.##..#.........',
 '.......#.#...#.....#...#.#......###..##..#.##....###.#...#...#.#..##....##...####.##.#.........#',
 '...................###..##.#.##.##..##.#.##..##.##.#....#.....#..#..#..#..#..#....#.##.....#..#.',
 '........#.##....#......#...#.......###.....#.....#..#.....#..##...#..#.#......#....#........#...',
     '#.##...#........#.....#...#...#....#..................##....#..#....#.#......#.#.#.......##.....',
 '....#.#..#...#......#...........#.............#.##..#..#...#.#....#..#...##...##...#.........#..',
 '..#.....#.#..#....#...#....#..............#............#.#.##..#...##.#.....#.###..#...#.......#',
 '....#.#.......#.##.....#.#......##....#...#.....#.##.####...#....#....##....##....####..#.....#.',
 '..#....#...#.#.##.....####...........#..........#..#.#.#..........#..##.##.#.###.#...##.#...#.#.',
 '##.##.#...#.##..#.#..#.##.##......#..#....##..#.#....##.#..#...#...#...#......#.......##...#..##',
 '..#...#....####.###....###......#.#..###...#..##......#....#.......#..#........#......#...#.....',
 '.......#..#..#..#..#..#.....##.#....##..###.##.##...####..#...##.##.#......#.#.#.##...#..#......',
    '...#.#..###.#.#...##...#...##.......##..#####.##.##.#.#.....##...#..#.#.###.#..##.#..###.......#',
 '...........##.............##........#.........#........#...#.#.#.....#.##..#..##.#..#.#.....#...',
 '.#.#..#........#.......#..#.....#..#...#.....#.##............#.#.#..###.#.##...#..##...#..#...#.',
 '##...#..#.##..#.##....###...#.....#.###...##.....#.#...##.......##..............#......####....#',
 '...#..##.#..#..#..##.#...............#...#.............#.#......#.#...#...#..#.#........#.###..#',
 '.....#...........##..#...#.#.#.#.........##.#.........###.#...#.#.#..#....#............#...#.#..',
 '...##..........##...##...#....#.#.....##...#..##.#.......#.#.##.......#........#.........##.#...',
 '##.......#.#........#....##...###....###.##...#.##........#.#.......#......#..........###.##....',
     '.#.........##..##....#..#..#..#..#.###..#......#.........####.#..#....#....##.#..##.#..####.....',
 '......#..##...............#..#............#..#.##......##..#...........#..#.########..#........#',
 '.#.###..#.#..#.#...#.#....#....#..#.#.#..#......##........#.......#...##..#.#.....##...#.......#',
 '..#....#..#..#...#....#...#..##.......##.........#........####.#.....#........#.#.......#.......',
 '##...........#..#.#..#...........#..#....#....#......#.....#.##............#......##..#..##.#...',
 '...#.#..##....##..#.#####..##....##....##...####......###.......#.##.##.#..##..#.###..#.#....##.',
 '....#..#.##..#..#.###...#..#.#..##.##.#..#..#.........#.#........#..#.##.##..#..#......#....##..',
 '.....#.#...##......#.#....#.#.#..##....#.##.........##....#......#....#.##............#.........',
     '#........#.........###.##...#..##....#....#......##.##....###......#......#..#...#..#........#..',
 '.....#......##..#.........#....#......#....##....#......#...##...........#.............#.#....#.',
 '#.##.....##...##....###...#....####..##....####...#.........#..#.##..##...###..###....###.#.....',
 '.##......#.#...#....#....######..##.#..#..#.#.#.....#...##....###.#####.###.#..#..#.##.#........',
 '......#..##...#.##.....#.......#..#......#.....#.......#..###...#......###.....#....#..........#',
 '##............#..#..#.#..#..#..#......#.....#..#.#..#..........#...#......#..#..#.....#.........',
 '...#....#...##.##........#...#..#..#..#.#.......#....#..#.#...#.##..........#.#..#.#............',
 '#.##.#...#....#....##....##....###..#...#......#..##.#.#...........##...#........#........#...#.',
     '.#.............#..##.#..#..#.##.#.##.##.#.....#..#.........####..#.#..#...##..#........#........',
 '.##.##..#....#........#...#.#.#...#..#..#....##..##.#..#..#.#....##....##.#..###....#.....##...#',
 '##.#.#..#....#........##.#.....#.......##......##.#.##......##..#.##.###.#..##..#....###.#.....#',
 '.#..............#....###.......#.#..#..#.......#......##....###.......#............##....#...###',
 '.#..###..##...####...####.##.........##.#.##..#.##....###.#.#.........#.#.#....##......#.....#..',
 '..#..##.#..#..#.##..##.#....##....#...#..#..##.#..#.##..#........#..#.....#..##........#..#...#.',
 '.........#..#........##.##....#......#..#......#...#.#.#...#.#.....#.....................#......',
 '..#...####.....#.#..#..#.#.......##....#....#.....#.#..........##.....##....##....###...###.#.##',
     '...............#.#..#.#.........##...#.....##.#..#......#..........#....#..#..#..#..#.###.....#.',
 '.#..#.#..............##..#.#.........#..#..#....##..#......#..#...##....##....#....#....##...##.',
 '..#...#....##..........#.....#....#..#..#.#..#.......#.#........#.......#....#...##....#.##..##.',
 '###..#.#...#.##......##..####....##...####..........#..#....#...#...###...#.#........##.##......',
 '#.###.#....#....##.#..##..#.###.#..#.##...###...###.#..#...#...#...##.#.##.......#...##....#....',
 '.#..#....##..#..#.#...##.##.......#......##.....#........#..........#...#.....#.........#.#.#.#.',
 '##...##......##..##......##.###.#.#..........#.....#...#.#.....##.......#.......##.....#........',
 '.#...#...#..........#.#..#...#....#...#.#......#.....###........#...#####.##..##........#....#..',
     '#....###....#.....#####....###...##..#.###..#...#......#......#...#.....#.......#...##.....#.#..',
 '........#..#.......##..#..#..##.##.#..#..#.#.......#..#....###...##....####.....#.....#.......#.',
 '................#...##..#.......##.#..#..#....#.....#..#..#..####.##.##..###.##..#....#..#.#.#..',
 '#..#...........#..#...........#..##...#.#.......#..#..#.#......##............#.##.#.....#....#..',
 '#....#.#.............###....#.#....##.#......###........#.#.................#..........#..#....#',
 '#..#.#.............#.#....#..####..#..................#.#.####..#.##.#...#.##....#..#.....##...#',
 '#...#...#..#..#..#.#....###.#........##.###..#....#...........#.#........####..#..#.....#....#..',
 '..#..#..###....##.#....#..#............#...#...#..##...#.....#.....#....#....##............#...#',
     '.....#..#.###..#.#..#....#.......##........#.#.###.#.###...####....##...##.....#..#......#..#...',
 '.#....#...#.#.#....#.##.#......##.#...#...#.##.#.##..####.##..#........#....##..#.##....###.....',
 '..#....#.......#..#.................#...##.#.#...#.....#...#..#.###..#..####..#..#.##..#........',
 '....#....###.....####..##..#.##.#.####......#...............#....#..#......##...#.#..#.#.....#..',
 '..#...#..#.........#..#..#..#..##.###.........###.....####.####.#.#.#...##.#............##.#..#.',
 '.........#.#.....##....#....#..............##...........#.........##............##...#....##....',
 '...###..#...##..###.....#.......#...#...##..##.##....#...####.#..#......#..##.#..###..#...#.###.',
 '.........#.###....##..#.#.....#.#..##....####.####.#.....#.......#....##.....##....##.......#...',
     '.#.#....#...#..#........#......#..#..#..#..##.#.#.#....#....#.#.###..#####.#.###....#.##....#...',
 '.#...#.#..##.#..#.....#.....#.##....##.#......#..#....#......#..#..#..#..#.##....#..#.#.#.#.....',
 '#..................#....##...##...##..##.....##....#.#....#...........#.........#.....##..#.##..',
 '#.........#.#..........##..#......###..#......#.#..#......#....##....#.....#..#.......#.#..#...#',
 '.........#....##.##.###...###.....##.#....#...#........#...##..#......#......#.#....#.......#...',
 '.#...#.#..#..#..#..####..####.....#...#..##.#..#.......#......#...##.#...#.##...###.#........#..',
 '.#.##..#.....#...#....##....#..##....#.#.#....#......#..#..#........#...............#....#......',
 '.#.#......#...##....#.....#......#.#..###..##...###....###.##....##....#.#...###.....##.#..#..#.',
     '....#....###...##........#....#....#...#..#..#..##.#..#......#..#.#...........#............#.#..',
 '..#.......#.#....##....#####.######.......#..##.#...........#...#.#...#.###...#.............#...',
 '......#...#..#..#..#..#..#..#...........#.#..#..##.......#............#...##...##..####.######..',
 '......##......#..#..#.#..........###...........#....##...#........#.#.#.#..####.##..##.#..#...#.',
 '..........#...#.................#..#....##.#.#.#.#.#........##.#.#...........#......#..#...#..#.',
 '.##.##...#...#....#..###....#..#..#..#....#......##....###.#......#.................#.#....#....',
 '#....#..#.#......#.##.#................#..#..#...#.##.#..#....................#.....#......#.#.#',
 '.##...####..##...####.#..#....#...........#..#..##..#.......#.....#.....#......##.#.#...#.....#.',
     '#.#..#..#..#..#..###..#...##...#...##......#..........#........#....#....#.#.#.#...#.#...#....#.',
 '.......#..........#...........###.....##..#.......#....#....#....#.##.........#.#........#......',
 '.......###.#......###......#..#...#.##.......#....#.#.#.....##..#.....##..#..#....##.#.....#..#.',
 '.........#..###..#..#..#....#.##....#.#..##.........#....##....##....####.#..#.........#....#...',
 '..#.#..####..##....###.##.##.##....##....###.....#...#..#.##.###.#..#.#.....##.#...#............',
 '#..#.###.#..##.#.##..#.#.#..##.#..##.##.#.#.#......#.....#..###.#.##...##..#..#.......##..##.#.#',
 '...........##........#.........................#.....#..........##..#.......##...#......##...##.',
 '..............#.#.##..........#.....#...##...##...#.....#.#..#......##....#..#..#.#...#..###...#']

gridmap = []
for line in seamap:
    gridmap.append(line.split())
    
gridmap = np.array(gridmap)
# gridmap = np.rot90(gridmap)
# gridmap = np.rot90(gridmap)
gridmap = np.flip(gridmap, 1)

seamap = [''.join(row) for row in gridmap]

sea_monster_0 = '..................#.'
sea_monster_1 = '#....##....##....###'
sea_monster_2 = '.#..#..#..#..#..#...'


for i in range(0, len(seamap)-2):
    for j in range(0, len(seamap[0])-19):
        if re.match(sea_monster_0, seamap[i][j:j+20]) and re.match(sea_monster_1, seamap[i+1][j:j+20]) and re.match(sea_monster_2, seamap[i+2][j:j+20]):
            temp = seamap[i][j:j+20]
            new_str = ""
            for k in range(20):
                if sea_monster_0[k] == '#':
                    new_str += 'O'
                else:
                    new_str += temp[k]

            seamap[i] = seamap[i][0:j] + new_str + seamap[i][j+20:] 
            
            temp = seamap[i+1][j:j+20]
            new_str = ""
            for k in range(20):
                if sea_monster_1[k] == '#':
                    new_str += 'O'
                else:
                    new_str += temp[k]

            seamap[i+1] = seamap[i+1][0:j] + new_str + seamap[i+1][j+20:] 
            
            temp = seamap[i+2][j:j+20]
            new_str = ""
            for k in range(20):
                if sea_monster_2[k] == '#':
                    new_str += 'O'
                else:
                    new_str += temp[k]

            seamap[i+2] = seamap[i+2][0:j] + new_str + seamap[i+2][j+20:] 

c = 0
for i in seamap:
    for j in i:
        if j == '#':
            c += 1
print(c)
