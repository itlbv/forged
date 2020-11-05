#include "Mob.h"
#include "Game.h"

Mob::Mob(double x, double y) : Entity(x, y), dest{0, 0} {}

void Mob::move() {
    velocity.set(0, 0);
    if (dest.x == 0 && dest.y == 0)
        return;
    if (pos.distanceTo(dest) < 0.1)
        return;
    velocity = pos.vectorTo(dest);
    velocity.normalize();
    velocity.setLength(velocity.length() * Game::delta * SPEED);
    pos.add(velocity.x, velocity.y);
}

void Mob::update() {
    move();
}

void Mob::setDest(double x, double y) {
    dest.set(x, y);
}
