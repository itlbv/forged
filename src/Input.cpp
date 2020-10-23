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

        if (event.type == SDL_KEYUP) {
            std::cout << event.key.keysym.sym << std::endl;
            switch (event.key.keysym.sym) {
                case SDLK_LEFT:
                    Game::player.setVelocity(0, 0);
                    break;
                case SDLK_RIGHT:
                    Game::player.setVelocity(0, 0);
                    break;
                case SDLK_UP:
                    Game::player.setVelocity(0, 0);
                    break;
                case SDLK_DOWN:
                    Game::player.setVelocity(0, 0);
                    break;
            }
        }

        if (event.type == SDL_KEYDOWN) {
            switch (event.key.keysym.sym) {
                case SDLK_w:
                    Game::viewport.y += 20;
                    break;
                case SDLK_s:
                    Game::viewport.y -= 20;
                    break;
                case SDLK_a:
                    Game::viewport.x += 20;
                    break;
                case SDLK_d:
                    Game::viewport.x -= 20;
                    break;
                case SDLK_z:
                    Game::zoomFactor -= 5;
                    break;
                case SDLK_x:
                    Game::zoomFactor += 5;
                    break;

                case SDLK_LEFT:
                    Game::player.setVelocity(-0.1, 0);
                    break;
                case SDLK_RIGHT:
                    Game::player.setVelocity(0.1, 0);
                    break;
                case SDLK_UP:
                    Game::player.setVelocity(0, -0.1);
                    break;
                case SDLK_DOWN:
                    Game::player.setVelocity(0, 0.1);
                    break;
            }
        }
    }
}
