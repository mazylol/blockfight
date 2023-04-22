//
// Created by mazylol on 4/21/23.
//

#include "player.h"
#include "raylib.h"

void MoveRight(Player *player) {
    player->PosX += 10;
}

void MoveLeft(Player *player) {
    player->PosX -= 10;
}

void DrawPlayer(Player *player, struct Color color) {
    DrawRectangle(player->PosX, player->PosY - 55, 50, 50, color);
}

void HandleMovement(Player *player, KeyboardKey left, KeyboardKey right) {
    if (player->PosX <= 0) {
        player->PosX = 0;
    }

    if (player->PosX >= 1280 - 50) {
        player->PosX = 1280 - 50;
    }

    if (IsKeyDown(left)) {
        MoveLeft(player);
    }

    if (IsKeyDown(right)) {
        MoveRight(player);
    }
}