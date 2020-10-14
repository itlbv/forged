#ifndef FORGED_RENDERWINDOW_H
#define FORGED_RENDERWINDOW_H
#include "SDL.h"

class RenderWindow {

    SDL_Window *window;
    SDL_Surface *surface;
    SDL_Renderer *renderer;

    SDL_Rect rect{};

public:
    RenderWindow(char *title, int width, int height);
    void render();
    void cleanUp();
};


#endif //FORGED_RENDERWINDOW_H
