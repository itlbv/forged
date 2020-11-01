#include "Player.h"
#include "Game.h"
#include <iostream>

Player::Player(double x, double y) : Entity(x, y) {}

void Player::update() {
    move();
    checkCollision();
}

void Player::checkCollision() {
    for (Entity &e : Game::entities) {
        double distance = pos.distanceTo(e.pos);
        if (distance < body.radius * 2) {
            double penetrationDistance = body.radius * 2 - distance;
            Vect collisionNormal = pos.vectorTo(e.pos);         // TODO should it be written with pointers?
            collisionNormal.setToLength(penetrationDistance);
            setVelocity(collisionNormal);
            move();
            std::cout << "Colliding " << penetrationDistance << std::endl;
        }
    }
}

void Player::move() {
    pos.x += velocity.x;
    pos.y += velocity.y;
    velocity.set(0, 0);
}

void Player::setVelocity(Vect& vel) {
    velocity.x = vel.x;
    velocity.y = vel.y;
}
