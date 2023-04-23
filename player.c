#include "player.h"
#include "raylib.h"

void MoveRight(Player *player) {
    player->PosX += 10;
}

void MoveLeft(Player *player) {
    player->PosX -= 10;
}

void Jump(Player *player) {
    player->PosY -= 150;
}

void DrawPlayer(Player *player, struct Color color) {
    DrawRectangle(player->PosX, player->PosY - 55, 50, 50, color);
}

void HandleMovement(Player *player, KeyboardKey left, KeyboardKey right, KeyboardKey jump) {
    if (player->PosX <= 0) {
        player->PosX = 0;
    }

    if (player->PosX >= 1280 - 50) {
        player->PosX = 1280 - 50;
    }

    if (player->PosY < 720) {
        player->PosY += 5;
    } else {
        player->IsJumping = false;
    }

    if (IsKeyDown(left)) {
        MoveLeft(player);
    }

    if (IsKeyDown(right)) {
        MoveRight(player);
    }

    if (IsKeyPressed(jump)) {
        if (player->IsJumping == false) {
            Jump(player);
            player->IsJumping = true;
        }
    }
}