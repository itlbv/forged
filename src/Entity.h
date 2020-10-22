#ifndef FORGED_ENTITY_H
#define FORGED_ENTITY_H

#include <SDL.h>

class Entity {
    float x, y;
    SDL_Rect rect{};

public:

    Entity(float x, float y);

    SDL_Rect *getRect();

    static int worldToScreen(float world);
};


#endif //FORGED_ENTITY_H
