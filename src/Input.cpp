#include "Input.h"
#include <SDL.h>
#include "Game.h"

#include <iostream>

SDL_Event Input::event;
Vect Input::velocityPlayer{0, 0};
SDL_Point Input::mousePos{0, 0};
Entity* Input::selectedEntity;
bool Input::playerMove = false;

void Input::getInput() {
    while (SDL_PollEvent(&event)) {
        if (event.type == SDL_QUIT) {
            Game::quit = true;
        }

        if (event.type == SDL_MOUSEMOTION) {
            mousePos = {event.motion.x, event.motion.y};
        }

        if (event.type == SDL_MOUSEBUTTONDOWN) {
            if (event.button.button == SDL_BUTTON_LEFT) {
                for (Entity &e : Game::entities) {
                    if (SDL_PointInRect(&mousePos, e.getDrawingRect())) {
                        selectedEntity = &e;
                    }
                }
            } else if (event.button.button == SDL_BUTTON_RIGHT) {
                if (selectedEntity != nullptr) {
                    selectedEntity->pos.add(0.1, 0.1);
                }
            }
        }

        if (event.type == SDL_MOUSEWHEEL) {
            //std::cout << event.wheel.x << " " << event.wheel.y << std::endl;
            //Game::viewport.y += 50;
        }

        velocityPlayer.set(0, 0);
        if (event.type == SDL_KEYUP) {
            std::cout << "ket up" << std::endl;
            playerMove = false;
            switch (event.key.keysym.sym) {
                case SDLK_LEFT:
                case SDLK_RIGHT:
                case SDLK_UP:
                case SDLK_DOWN:
                    //pvelocityPlayer.set(0, 0);
                    break;
            }
        }

        if (event.type == SDL_KEYDOWN) {
            std::cout << "key down" << std::endl;
            playerMove= true;
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
                    velocityPlayer.add(-0.1, 0);
                    break;
                case SDLK_RIGHT:
                    velocityPlayer.add(0.1, 0);
                    break;
                case SDLK_UP:
                    velocityPlayer.add(0, -0.1);
                    break;
                case SDLK_DOWN:
                    velocityPlayer.add(0, 0.1);
                    break;
            }
        }
        Game::player.setVelocity(velocityPlayer);
    }
}
