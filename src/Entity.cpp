#include "Entity.h"
#include "RenderWindow.h"

Entity::Entity(int x, int y) {
    rect.x = x;
    rect.y = y;
    rect.w  = 30;
    rect.h  = 30;
}

void Entity::moveTo(int x, int y) {
    rect.x = x;
    rect.y = y;
}

SDL_Rect* Entity::getRect() {
    return &rect;
}
