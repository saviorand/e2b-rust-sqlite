```python
import pygame
from src.objects import ObjectData
from src.player import Player
from src.enemies import Enemy

class Level:
    def __init__(self, player, level_limit=-1000):
        self.object_list = pygame.sprite.Group()
        self.enemy_list = pygame.sprite.Group()
        self.player = player
        self.level_limit = level_limit
        self.world_shift = 0

    def update(self):
        self.object_list.update()
        self.enemy_list.update()

    def draw(self, screen):
        screen.fill((0, 0, 0))
        self.object_list.draw(screen)
        self.enemy_list.draw(screen)

    def shift_world(self, shift_x):
        self.world_shift += shift_x
        for object in self.object_list:
            object.rect.x += shift_x
        for enemy in self.enemy_list:
            enemy.rect.x += shift_x

class Level_01(Level):
    def __init__(self, player):
        Level.__init__(self, player)
        self.level_limit = -1500
        level = [
            [210, 70, 500, 500],
            [210, 70, 800, 400],
            [210, 70, 1000, 500],
            [210, 70, 1120, 280],
        ]
        enemies = [
            [500, 500],
            [800, 400],
            [1000, 500],
            [1120, 280],
        ]

        for object in level:
            block = ObjectData(object[0], object[1], object[2], object[3])
            block.player = self.player
            block.level = self
            self.object_list.add(block)

        for enemy in enemies:
            enemy = Enemy(enemy[0], enemy[1])
            enemy.player = self.player
            enemy.level = self
            self.enemy_list.add(enemy)
```