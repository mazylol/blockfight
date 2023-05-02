#ifndef BLOCKFIGHT_PLAYER_H
#define BLOCKFIGHT_PLAYER_H

#endif //BLOCKFIGHT_PLAYER_H

#include "raylib.h"

typedef struct {
    Texture2D Texture;
    bool IsMoving;
} Weapon;

typedef struct {
    int PosX;
    int PosY;
    int Health;
    bool IsJumping;
    Weapon Sword;
    KeyboardKey LastKey;
} Player;

void DrawPlayer(Player *player, struct Color color);

void HandleMovement(Player *player, KeyboardKey left, KeyboardKey right, KeyboardKey jump);

void HandleCollisions(Player *playerOne, Player *playerTwo);