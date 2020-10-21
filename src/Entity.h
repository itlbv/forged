#ifndef FORGED_ENTITY_H
#define FORGED_ENTITY_H

#include <SDL.h>

class Entity {
    SDL_Rect rect{};
public:
    Entity(int x, int y);

    void moveTo(int x, int y);

    SDL_Rect *getRect();
};


#endif //FORGED_ENTITY_H
