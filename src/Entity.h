#ifndef FORGED_ENTITY_H
#define FORGED_ENTITY_H

#include <SDL.h>
#include "Body.h"
#include "Vect.h"

class Entity {
    SDL_Rect rect{};
public:
    Vect pos;
    Body body{0.24f};

    Entity(float x, float y);

    SDL_Rect *getRect();

    static int worldToScreen(float world);
};


#endif //FORGED_ENTITY_H
