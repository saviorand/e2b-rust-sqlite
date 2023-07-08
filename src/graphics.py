```python
import pygame
from src.game import GameState
from src.player import Player
from src.enemies import Enemy
from src.levels import Level
from src.objects import Object

# Initialize Pygame
pygame.init()

# Set the dimensions of the game window
SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))

# Load image assets
player_image = pygame.image.load('assets/images/player.png')
enemy_image = pygame.image.load('assets/images/enemies.png')
level_image = pygame.image.load('assets/images/levels.png')
object_image = pygame.image.load('assets/images/objects.png')

def draw_game_state(game_state):
    """Draw the current game state to the screen."""
    # Clear the screen
    screen.fill((0, 0, 0))

    # Draw the level
    for tile in game_state.level.tiles:
        screen.blit(level_image, tile.rect)

    # Draw the objects
    for obj in game_state.objects:
        screen.blit(object_image, obj.rect)

    # Draw the enemies
    for enemy in game_state.enemies:
        screen.blit(enemy_image, enemy.rect)

    # Draw the player
    screen.blit(player_image, game_state.player.rect)

    # Update the display
    pygame.display.flip()

def draw_text(text, x, y, color=(255, 255, 255)):
    """Draw text to the screen."""
    font = pygame.font.Font(None, 36)
    text = font.render(text, True, color)
    screen.blit(text, (x, y))
```
