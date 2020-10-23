#include "Player.h"
#include "Game.h"
#include <cmath>
#include <iostream>

Player::Player(float x, float y) : Entity(x, y) {}

void Player::update() {
    move();
    checkCollision();
}

void Player::checkCollision() {
    for (Entity &e : Game::entities) {
        if (distanceBetween(pos, e.pos) < body.radius * 2)
        std::cout << "Colliding" << std::endl;
    }
}

float Player::distanceBetween(Vect &vect1, Vect &vect2) {
    using namespace std;
    return sqrt(pow(abs(vect1.x - vect2.x), 2) + pow(abs(vect1.y - vect2.y), 2));
}

void Player::move() {
    pos.x += velocity.x;
    pos.y += velocity.y;
    //setVelocity(0, 0);
}

void Player::setVelocity(float x, float y) {
    velocity.x = x;
    velocity.y = y;
}
