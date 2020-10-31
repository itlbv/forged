#pragma once

#include <SDL.h>
#include "Body.h"
#include "Vect.h"

class Entity {
    SDL_Rect drawingRect{};
public:
    Vect pos;
    Body body{0.24f};

    Entity(double x, double y);

    SDL_Rect *getDrawingRect();

    static int worldToScreen(double world);
};
