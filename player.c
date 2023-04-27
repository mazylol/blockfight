#include "player.h"
#include "raylib.h"

void MoveRight(Player *player) {
    player->PosX += 10;
}

void MoveLeft(Player *player) {
    player->PosX -= 10;
}

void Jump(Player *player) {
    player->IsJumping = true;
}

void DrawPlayer(Player *player, Texture2D sword, struct Color color) {
    ;

    DrawRectangle(player->PosX, player->PosY - 55, 50, 50, color);
    DrawTexture(sword, player->PosX, player->PosY - 55, WHITE);
}

void HandleMovement(Player *player, KeyboardKey left, KeyboardKey right, KeyboardKey jump) {
    if (player->PosX <= 0) {
        player->PosX = 0;
    }

    if (player->PosX >= 1280 - 50) {
        player->PosX = 1280 - 50;
    }

    if (player->PosY < 720 && !player->IsJumping) {
        player->PosY += 5;
    }

    if (player->IsJumping && player->PosY != 720 - 150) {
        player->PosY -= 10;
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
        if (player->PosY > 719) {
            player->IsJumping = true;
        }
    }
}