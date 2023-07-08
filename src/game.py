```python
import pygame
from player import Player
from enemies import Enemy
from levels import Level
from objects import Object
from physics import Physics
from graphics import Graphics
from sounds import Sounds

class Game:
    def __init__(self):
        self.state = {
            'score': 0,
            'lives': 3,
            'level': 1,
        }
        self.player = Player()
        self.enemies = Enemy()
        self.level = Level()
        self.objects = Object()
        self.physics = Physics()
        self.graphics = Graphics()
        self.sounds = Sounds()

    def run(self):
        while True:
            self.graphics.draw(self.player, self.enemies, self.level, self.objects)
            self.physics.update(self.player, self.enemies, self.level, self.objects)
            self.sounds.play()

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    quit()

            keys = pygame.key.get_pressed()
            if keys[pygame.K_LEFT]:
                self.player.move_left()
            if keys[pygame.K_RIGHT]:
                self.player.move_right()
            if keys[pygame.K_SPACE]:
                self.player.jump()

            self.state['score'] += self.objects.collect(self.player)
            self.state['lives'] -= self.enemies.collide(self.player)

            if self.state['lives'] <= 0:
                print("Game Over")
                pygame.quit()
                quit()

            pygame.display.update()

if __name__ == "__main__":
    game = Game()
    game.run()
```