#include "Entity.h"
#include "RenderWindow.h"

Entity::Entity(float xa, float ya)
        : x(xa), y(ya) {
    rect.w = worldToScreen(0.49f);
    rect.h = worldToScreen(0.49f);
}

SDL_Rect *Entity::getRect() {
    rect.x = worldToScreen(x);
    rect.y = worldToScreen(y);
    return &rect;
}

int Entity::worldToScreen(float world) {
    return world * 50;
}
