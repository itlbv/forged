#ifndef FORGED_MAP_H
#define FORGED_MAP_H

#include "RenderWindow.h"

class Map {
    RenderWindow& window;
public:
    Map(RenderWindow &window);

    void render();
};


#endif //FORGED_MAP_H
