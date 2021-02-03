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
    checkCollision();
}

void Mob::checkCollision() {
    for (Mob &m : Game::mobs) {
        if (this == &m)
            return;
        double distance = pos.distanceTo(m.pos);
        if (distance < body.radius * 2) {
            double penetrationDistance = body.radius * 2 - distance;
            Vect collisionNormal = pos.vectorTo(m.pos);         // TODO should it be written with pointers?
            collisionNormal.setLength(penetrationDistance);
            setVelocity(collisionNormal);
            move();
        }
    }
}

void Mob::setVelocity(Vect& vel) {
    velocity.x = vel.x;
    velocity.y = vel.y;
}

void Mob::setDest(double x, double y) {
    dest.set(x, y);
}
