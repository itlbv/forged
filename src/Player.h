#ifndef FORGED_PLAYER_H
#define FORGED_PLAYER_H

#include "Entity.h"
#include "Vect.h"

class Player : public Entity {

    void move();

public:
    Vect velocity{0, 0};

    Player(float x, float y);

    void update();

    void setVelocity(float x, float y);
};


#endif //FORGED_PLAYER_H
