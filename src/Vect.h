#ifndef FORGED_VECT_H
#define FORGED_VECT_H

struct Vect {
    float x, y;

    void operator+(const Vect& vect){
        this->x += vect.x;
        this->y += vect.y;
    };
};

#endif //FORGED_VECT_H
