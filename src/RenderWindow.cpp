#include "RenderWindow.h"

#include <SDL.h>
#include <iostream>

RenderWindow::RenderWindow(const char *title, int width, int height)
        : window(SDL_CreateWindow(title, SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, width, height, 0)),
          renderer(SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED)) {
    if (window == nullptr)
        std::cout << "SDL_CreateWindow FAILED. Error: " << SDL_GetError();

    if (renderer == nullptr)
        std::cout << "SDL_CreateRenderer FAILED. Error: " << SDL_GetError();
}

void RenderWindow::startFrame() {
    SDL_SetRenderDrawColor(renderer, 90, 125, 70, 255); // set default green
    SDL_RenderClear(renderer);
}

void RenderWindow::render(Entity* entity) {
    SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);
    SDL_RenderFillRect(renderer, entity->getRect());
}

void RenderWindow::renderToScreen() {
    SDL_RenderPresent(renderer);
}

void RenderWindow::cleanUp() {
    SDL_DestroyWindow(window);
}
