#include "Input.h"
#include <SDL.h>
#include "Game.h"

SDL_Event Input::event;

void Input::getInput() {
    while (SDL_PollEvent(&event)) {
        if (event.type == SDL_QUIT) {
            Game::quit = true;
        }
    }
}
