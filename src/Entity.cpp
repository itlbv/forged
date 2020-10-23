#include "Entity.h"
#include "RenderWindow.h"
#include "Game.h"

Entity::Entity(float x, float y)
        : pos(Vect{x, y}) {
    rect.w = worldToScreen(0.49f);
    rect.h = worldToScreen(0.49f);
}

SDL_Rect *Entity::getRect() {
    rect.x = worldToScreen(pos.x);
    rect.y = worldToScreen(pos.y);
    rect.w = worldToScreen(0.49f);
    rect.h = worldToScreen(0.49f);
    return &rect;
}

int Entity::worldToScreen(float world) {
    return world * Game::zoomFactor;
}
