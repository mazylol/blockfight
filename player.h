//
// Created by mazylol on 4/21/23.
//

#ifndef BLOCKFIGHT_PLAYER_H
#define BLOCKFIGHT_PLAYER_H

#endif //BLOCKFIGHT_PLAYER_H

#include "raylib.h"

typedef struct {
    int PosX;
    int PosY;
    int Health;
} Player;

void DrawPlayer(Player *player, struct Color color);

void HandleMovement(Player *player, KeyboardKey left, KeyboardKey right);