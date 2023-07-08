```python
import pygame
from src.physics import apply_gravity, check_collision

class Enemy:
    def __init__(self, x, y, speed, image_path):
        self.x = x
        self.y = y
        self.speed = speed
        self.image = pygame.image.load(image_path)
        self.rect = self.image.get_rect()
        self.rect.topleft = (x, y)

    def move(self, level_layout):
        self.x += self.speed
        self.rect.topleft = (self.x, self.y)
        if check_collision(self.rect, level_layout):
            self.speed = -self.speed
            self.x += self.speed
            self.rect.topleft = (self.x, self.y)

    def update(self, level_layout):
        apply_gravity(self)
        self.move(level_layout)

goomba = Enemy(100, 100, 2, "assets/images/enemies.png")
```
