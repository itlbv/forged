#include "RenderWindow.h"

#include <SDL.h>
#include <iostream>

RenderWindow::RenderWindow(char *title, int width, int height)
: window(nullptr), renderer(nullptr)
{
    window = SDL_CreateWindow(title,
                              SDL_WINDOWPOS_UNDEFINED,
                              SDL_WINDOWPOS_UNDEFINED,
                              width,
                              height,
                              SDL_WINDOW_SHOWN);
    if (window == nullptr)
        std::cout << "SDL_CreateWindow FAILED. Error: " << SDL_GetError();

    surface = SDL_GetWindowSurface(window);
    if (surface == nullptr)
        std::cout << "SDL_GetWindowSurface FAILED. Error: " << SDL_GetError();

    renderer = SDL_CreateRenderer(window,
                                  -1,
                                  SDL_RENDERER_ACCELERATED);
    if (renderer == nullptr)
        std::cout << "SDL_CreateRenderer FAILED. Error: " << SDL_GetError();

    rect.x = 50;
    rect.y = 50;
    rect.w = 50;
    rect.h = 50;
}

void RenderWindow::render() {
    SDL_FillRect(surface, &rect, SDL_MapRGB(surface->format, 10, 200, 120));
    SDL_UpdateWindowSurface(window);
}

void RenderWindow::cleanUp() {
    SDL_FreeSurface(surface);
    SDL_DestroyWindow(window);
}
