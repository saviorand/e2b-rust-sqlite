```python
import pygame
from src.player import Player
from src.enemies import Enemy
from src.levels import Level
from src.objects import Object

GRAVITY = 0.5

def apply_gravity(entity):
    if isinstance(entity, (Player, Enemy, Object)):
        if not entity.on_ground:
            entity.velocity_y += GRAVITY

def check_collision(entity1, entity2):
    rect1 = pygame.Rect(entity1.x, entity1.y, entity1.width, entity1.height)
    rect2 = pygame.Rect(entity2.x, entity2.y, entity2.width, entity2.height)
    return rect1.colliderect(rect2)

def handle_collision(player, entity):
    if isinstance(entity, Enemy):
        if player.velocity_y > 0 and check_collision(player, entity):
            player.jump()
            entity.die()
        elif check_collision(player, entity):
            player.die()
    elif isinstance(entity, Object):
        if check_collision(player, entity):
            entity.collect()

def update_positions(entities, level):
    for entity in entities:
        entity.x += entity.velocity_x
        entity.y += entity.velocity_y
        if isinstance(entity, (Player, Enemy)):
            if entity.x < 0 or entity.x > level.width:
                entity.velocity_x *= -1
            if entity.y < 0 or entity.y > level.height:
                entity.on_ground = True
                entity.velocity_y = 0
            else:
                entity.on_ground = False
```