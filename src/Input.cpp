#include "Input.h"
#include <SDL.h>
#include "Game.h"

#include <iostream>

SDL_Event Input::event;

void Input::getInput() {
    while (SDL_PollEvent(&event)) {
        if (event.type == SDL_QUIT) {
            Game::quit = true;
        }
        if (event.type == SDL_MOUSEWHEEL) {
            //std::cout << event.wheel.x << " " << event.wheel.y << std::endl;
            //Game::viewport.y += 50;
        }
        if (event.type == SDL_KEYDOWN) {
            switch (event.key.keysym.sym) {
                case 119: // W
                    Game::viewport.y += 20;
                    break;
                case 115: // S
                    Game::viewport.y -= 20;
                    break;
                case 97:  // A
                    Game::viewport.x += 20;
                    break;
                case 100: // D
                    Game::viewport.x -= 20;
                    break;
                case 122: // Z
                    Game::zoomFactor -= 5;
                    break;
                case 120: // X
                    Game::zoomFactor += 5;
                    break;
            }
        }
    }
}
