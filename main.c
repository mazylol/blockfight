#include "raylib.h"
#include "player.h"

int main(void) {
    const int screenWidth = 1280;
    const int screenHeight = 720;

    InitWindow(screenWidth, screenHeight, "Block Fight");

    SetTargetFPS(60);

    Player playerOne = {0, screenHeight, 100, false};
    Player playerTwo = {screenWidth - 50, screenHeight, 100, false};

    Texture2D sword = LoadTexture("C:/Users/LP/GolandProjects/blockfight/resources/sword.png");

    bool gameStarted = false;

    while (!WindowShouldClose()) {
        if (!gameStarted) {
            if (IsKeyPressed(KEY_ENTER)) {
                gameStarted = true;
            }

            BeginDrawing();

            ClearBackground(RAYWHITE);
            DrawText("Block Fight!!!", screenWidth / 4, screenHeight / 3, 100, CLITERAL(Color) {137, 207, 240, 255});

            EndDrawing();
        } else {

            HandleMovement(&playerOne, KEY_A, KEY_D, KEY_W);
            HandleMovement(&playerTwo, KEY_LEFT, KEY_RIGHT, KEY_UP);

            BeginDrawing();

            ClearBackground(BLACK);

            DrawPlayer(&playerOne, sword, RED);
            DrawPlayer(&playerTwo, sword, BLUE);

            EndDrawing();
        }
    }

    UnloadTexture(sword);

    CloseWindow();

    return 0;
}