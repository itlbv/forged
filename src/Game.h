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
    static SDL_Rect viewport;
    static int zoomFactor;

    Game();

    void run(unsigned int deltaTime);
};


#endif //FORGED_GAME_H
