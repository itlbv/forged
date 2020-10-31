#include "Entity.h"
#include "RenderWindow.h"
#include "Game.h"

Entity::Entity(double x, double y)
        : pos(Vect{x, y}) {
    drawingRect.w = worldToScreen(0.49);
    drawingRect.h = worldToScreen(0.49);
}

SDL_Rect *Entity::getDrawingRect() {
    drawingRect.x = worldToScreen(pos.x) - drawingRect.w/2;
    drawingRect.y = worldToScreen(pos.y) - drawingRect.h/2;
    drawingRect.w = worldToScreen(0.49);
    drawingRect.h = worldToScreen(0.49);
    return &drawingRect;
}

int Entity::worldToScreen(double world) {
    return world * Game::zoomFactor;
}
