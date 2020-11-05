#pragma once

#include <cmath>

struct Vect {
    double x, y;

    double length() const {
        using namespace std;
        return sqrt(pow(x, 2) + pow(y, 2));
    };

    double distanceTo(Vect &vect) const {
        return sqrt(pow(abs(x - vect.x), 2) + pow(abs(y - vect.y), 2));
    }

    Vect vectorTo(Vect &vect) const {
        Vect vectorTo{};
        vectorTo.x = vect.x - x;
        vectorTo.y = vect.y - y;
        return vectorTo;

    }

    void normalize() {
        double length = this->length();
        if (length > 0) {
            x /= length;
            y /= length;
        }
    }

    void setLength(double desiredLength) {
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

    void add(double xa, double ya){
        x += xa;
        y += ya;
    }
};
