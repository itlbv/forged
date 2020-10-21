#ifndef FORGED_GAME_H
#define FORGED_GAME_H

#include <SDL.h>
#include <vector>
#include "Entity.h"
#include "RenderWindow.h"

class Game {
    RenderWindow window;
    Entity player;
    std::vector<Entity> entities;

    void createEntities();

    void renderEntities();

public:

    static bool quit;

    Game();

    void run(float deltaTime);
};


#endif //FORGED_GAME_H
