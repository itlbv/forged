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

    void updateViewport(SDL_Rect* viewport);

    void startFrame();

    void renderEntity(Entity& e);

    void renderLine(int x1, int y1, int x2, int y2);

    void showFrame();

    ~RenderWindow();
};


#endif //FORGED_RENDERWINDOW_H
