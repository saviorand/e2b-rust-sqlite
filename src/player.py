```python
import pygame
from src.physics import apply_gravity, detect_collision
from src.graphics import draw_player
from src.sounds import play_jump_sound

class Player:
    def __init__(self, x, y, width, height):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.vel = 10
        self.is_jump = False
        self.jump_count = 10
        self.left = False
        self.right = False
        self.walk_count = 0
        self.standing = True
        self.hitbox = (self.x + 17, self.y + 11, 29, 52)

    def draw(self, win):
        draw_player(self, win)

    def move(self, keys):
        if keys[pygame.K_LEFT] and self.x > self.vel:
            self.x -= self.vel
            self.left = True
            self.right = False
            self.standing = False
        elif keys[pygame.K_RIGHT] and self.x < 500 - self.width - self.vel:
            self.x += self.vel
            self.right = True
            self.left = False
            self.standing = False
        else:
            self.standing = True
            self.walk_count = 0

        if not self.is_jump:
            if keys[pygame.K_SPACE]:
                self.is_jump = True
                self.right = False
                self.left = False
                self.walk_count = 0
                play_jump_sound()
        else:
            if self.jump_count >= -10:
                neg = 1
                if self.jump_count < 0:
                    neg = -1
                self.y -= (self.jump_count ** 2) * 0.5 * neg
                self.jump_count -= 1
            else:
                self.is_jump = False
                self.jump_count = 10

    def update(self):
        self.y = apply_gravity(self.y)
        self.hitbox = (self.x + 17, self.y + 11, 29, 52)

    def check_collision(self, enemies):
        for enemy in enemies:
            if detect_collision(self.hitbox, enemy.hitbox):
                return True
        return False
```