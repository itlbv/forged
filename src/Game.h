#pragma once

#include <SDL.h>
#include <vector>
#include "Entity.h"
#include "RenderWindow.h"
#include "Player.h"
#include "Mob.h"

class Game {
    RenderWindow window;

    void updateMobs();

    void createMobs();

    void renderEntities();

public:
    static bool quit;
    static SDL_Rect viewport;
    static int zoomFactor;
    static std::vector<Mob> mobs;
    static Player player;
    static double delta;

    Game();

    void run(unsigned int deltaTime);
};
