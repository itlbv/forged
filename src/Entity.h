#ifndef FORGED_ENTITY_H
#define FORGED_ENTITY_H

#include <SDL.h>

class Entity {
    float x, y;
    SDL_Rect rect{};

    static int worldToScreen(float world);

public:
    Entity(float x, float y);

    SDL_Rect *getRect();
};


#endif //FORGED_ENTITY_H
