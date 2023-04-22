#include "raylib.h"
#include "player.h"

int main(void) {
    const int screenWidth = 1280;
    const int screenHeight = 720;

    InitWindow(screenWidth, screenHeight, "Block Fight");

    SetTargetFPS(60);

    Player playerOne = {0, screenHeight, 100};
    Player playerTwo = {screenWidth - 50, screenHeight, 100};

    while (!WindowShouldClose()) {
        HandleMovement(&playerOne, KEY_A, KEY_D);
        HandleMovement(&playerTwo, KEY_LEFT, KEY_RIGHT);

        BeginDrawing();

        ClearBackground(BLACK);

        DrawPlayer(&playerOne, RED);
        DrawPlayer(&playerTwo, BLUE);

        DrawFPS(1, 1);

        EndDrawing();
    }

    CloseWindow();

    return 0;
}