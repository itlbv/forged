#include "Input.h"
#include <SDL.h>
#include "Game.h"

#include <iostream>

SDL_Event Input::event;
Vect Input::velocityPlayer{0, 0};

void Input::getInput() {
    while (SDL_PollEvent(&event)) {
        if (event.type == SDL_QUIT) {
            Game::quit = true;
        }
        if (event.type == SDL_MOUSEWHEEL) {
            //std::cout << event.wheel.x << " " << event.wheel.y << std::endl;
            //Game::viewport.y += 50;
        }

        velocityPlayer.set(0,0);
        if (event.type == SDL_KEYUP) {
            switch (event.key.keysym.sym) {
                case SDLK_LEFT:
                case SDLK_RIGHT:
                case SDLK_UP:
                case SDLK_DOWN:
                    velocityPlayer.set(0, 0);
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
                    velocityPlayer.set(-0.1, 0);
                    break;
                case SDLK_RIGHT:
                    velocityPlayer.set(0.1, 0);
                    break;
                case SDLK_UP:
                    velocityPlayer.set(0, -0.1);
                    break;
                case SDLK_DOWN:
                    velocityPlayer.set(0, 0.1);
                    break;
            }
        }
        Game::player.setVelocity(velocityPlayer);
    }
}
