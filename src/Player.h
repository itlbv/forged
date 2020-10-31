#pragma once

#include "Entity.h"
#include "Vect.h"

class Player : public Entity {

    void move();

    void checkCollision();

public:
    Vect velocity{0, 0};

    Player(double x, double y);

    void update();

    void setVelocity(double x, double y);
};
