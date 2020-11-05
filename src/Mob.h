#pragma once
#include "Entity.h"

class Mob : public Entity {
    Vect dest{};
    Vect velocity{};
    const int SPEED = 5;

    void move();

public:
    Mob(double x, double y);

    void update();

    void setDest(double x, double y);
};
