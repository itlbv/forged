#include "Entity.h"
#include "RenderWindow.h"
#include "Game.h"

Entity::Entity(float xa, float ya)
        : x(xa), y(ya) {
    rect.w = worldToScreen(0.49f);
    rect.h = worldToScreen(0.49f);
}

SDL_Rect *Entity::getRect() {
    rect.x = worldToScreen(x);
    rect.y = worldToScreen(y);
    rect.w = worldToScreen(0.49f);
    rect.h = worldToScreen(0.49f);
    return &rect;
}

int Entity::worldToScreen(float world) {
    return world * Game::zoomFactor;
}
