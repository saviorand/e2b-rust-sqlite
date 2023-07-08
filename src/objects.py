```python
import pygame
from src.physics import apply_gravity, check_collision

class GameObject:
    def __init__(self, x, y, width, height, image_path):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.image = pygame.image.load(image_path)
        self.rect = self.image.get_rect()

    def draw(self, window):
        window.blit(self.image, (self.x, self.y))

class Coin(GameObject):
    def __init__(self, x, y):
        super().__init__(x, y, 10, 10, "assets/images/coin.png")
        self.collected = False

    def collect(self, player):
        if check_collision(self, player):
            self.collected = True

class PowerUp(GameObject):
    def __init__(self, x, y, type):
        super().__init__(x, y, 20, 20, f"assets/images/{type}.png")
        self.type = type
        self.collected = False

    def collect(self, player):
        if check_collision(self, player):
            self.collected = True
            if self.type == "mushroom":
                player.grow()

def update_objects(objects, player):
    for obj in objects:
        apply_gravity(obj)
        obj.collect(player)
```