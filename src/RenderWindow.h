#ifndef FORGED_RENDERWINDOW_H
#define FORGED_RENDERWINDOW_H

#include "SDL.h"
#include "Entity.h"

class RenderWindow {
private:
    SDL_Window *window;
    SDL_Renderer *renderer;
public:
    RenderWindow(const char *title, int width, int height);

    void startFrame();

    void render(Entity* entity);

    void renderToScreen();

    void cleanUp();
};


#endif //FORGED_RENDERWINDOW_H
