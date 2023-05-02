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

void DrawPlayer(Player *player, struct Color color) {
    DrawRectangle(player->PosX, player->PosY - 55, 50, 50, color);

    if (player->LastKey == KEY_A || player->LastKey == KEY_LEFT) {
        DrawTexture(player->Sword.Texture, player->PosX - 30, player->PosY - 70, WHITE);
    } else if (player->LastKey == KEY_D || player->LastKey == KEY_RIGHT) {
        DrawTexture(player->Sword.Texture, player->PosX + 30, player->PosY - 70, WHITE);
    }
    //DrawTexture(player->Sword, player->PosX + 30, player->PosY - 70, WHITE);
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
        player->LastKey = left;
    }

    if (IsKeyDown(right)) {
        MoveRight(player);
        player->LastKey = right;
    }

    if (IsKeyPressed(jump)) {
        if (player->PosY > 719) {
            player->IsJumping = true;
        }
    }
}

void HandleCollisions(Player *playerOne, Player *playerTwo) {
    Rectangle playerOneRect = {
            playerOne->PosX,
            playerOne->PosY - 55,
            50,
            50
    };

    Rectangle playerTwoRect = {
            playerTwo->PosX,
            playerTwo->PosY - 55,
            50,
            50
    };

    if (CheckCollisionRecs(playerOneRect, playerTwoRect)) {
        playerOne->PosX -= 10;
        playerTwo->PosX += 10;
    }
}