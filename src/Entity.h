#ifndef FORGED_ENTITY_H
#define FORGED_ENTITY_H

#include <SDL.h>

class Entity {
    SDL_Rect rect{};
public:
    float x, y;

    Entity(float x, float y);

    SDL_Rect *getRect();

    static int worldToScreen(float world);
};


#endif //FORGED_ENTITY_H
