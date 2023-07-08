import pygame

pygame.mixer.init()

def play_sound(sound_file):
    pygame.mixer.Sound(sound_file).play()

def play_music(music_file, loop=False):
    pygame.mixer.music.load(music_file)
    pygame.mixer.music.play(-1 if loop else 0)

def stop_music():
    pygame.mixer.music.stop()

jump_sound = pygame.mixer.Sound('assets/sounds/jump.wav')
coin_sound = pygame.mixer.Sound('assets/sounds/coin.wav')
death_sound = pygame.mixer.Sound('assets/sounds/death.wav')
theme_music = 'assets/sounds/theme.mp3'