#include "Player.h"

Player::Player(float x, float y) : Entity(x, y) {}

void Player::update() {
    move();
}

void Player::move() {
    x += velocity.x;
    y += velocity.y;
    //setVelocity(0, 0);
}

void Player::setVelocity(float x, float y) {
    velocity.x = x;
    velocity.y = y;
}
