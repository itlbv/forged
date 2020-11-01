#pragma once

#include <cmath>

struct Vect {
    double x, y;

    double length() {
        using namespace std;
        return sqrt(pow(x, 2) + pow(y, 2));
    };

    double distanceTo(Vect &vect) {
        return sqrt(pow(abs(x - vect.x), 2) + pow(abs(y - vect.y), 2));
    }

    Vect vectorTo(Vect &vect) {
        Vect vectorTo{};
        vectorTo.x = x - vect.x;
        vectorTo.y = y - vect.y;
        return vectorTo;

    }

    void normalize() {
        double length = this->length();
        if (length > 0) {
            x /= length;
            y /= length;
        }
    }

    void setToLength(double desiredLength) {
        double length = this->length();
        if (length > 0) {
            double newLength = desiredLength / length;
            x *= newLength;
            y *= newLength;
        }
    }

    void set(double xa, double ya){
        x = xa;
        y = ya;
    }
};
