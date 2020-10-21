#include <iostream>
#include <SDL.h>
#include <algorithm>
#include "RenderWindow.h"
#include "Game.h"

#define MILLISECONDS_PER_FRAME 16

int main(int argc, char **argv) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
        std::cout << "SDL_Init FAILED. Error: " << SDL_GetError() << std::endl;

    unsigned int prevFrameTime, timePassed, deltaTime = 0;
    prevFrameTime = SDL_GetTicks();
    Game game;
    while (!Game::quit) {

        timePassed = SDL_GetTicks() - prevFrameTime;
        prevFrameTime = SDL_GetTicks();
        deltaTime += timePassed;
        if (deltaTime < MILLISECONDS_PER_FRAME)
            continue;

        game.run(deltaTime);

        deltaTime = 0;
    }

    SDL_Quit();

    return 0;
}
