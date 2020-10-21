#include <iostream>
#include <SDL.h>
#include <algorithm>
#include "RenderWindow.h"
#include "Game.h"

int main(int argc, char **argv) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
        std::cout << "SDL_Init FAILED. Error: " << SDL_GetError() << std::endl;

    int oldTime = SDL_GetTicks();
    float dt = 1 / 60.0;

    Game game;
    while (!Game::quit) {
        int newTime = SDL_GetTicks();
        float frameTime = (float) (newTime - oldTime);
        oldTime = newTime;

        float deltaTime = std::min(dt, frameTime);
        std::cout << newTime << std::endl;
        game.run(deltaTime);
    }

    SDL_Quit();

    return 0;
}
