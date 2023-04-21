#include "raylib.h"

int main(void)
{
    const int screenWidth = 1280;
    const int screenHeight = 720;

    typedef struct Players {
        int PosX;
        int PosY;
        int Health;
    } Player;

    Player playerOne;
    playerOne.PosX = 100;
    playerOne.PosY = screenHeight;

    InitWindow(screenWidth, screenHeight, "raylib [core] example - basic window");

    SetTargetFPS(60);

    while (!WindowShouldClose())
    {
        BeginDrawing();

        ClearBackground(BLACK);

        DrawRectangle(playerOne.PosX, playerOne.PosY - 55, 50, 50, MAROON);

        DrawFPS(1,1);

        EndDrawing();
    }

    CloseWindow();
    return 0;
}