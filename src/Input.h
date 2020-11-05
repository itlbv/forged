#pragma once

#include <SDL.h>
#include "Vect.h"
#include "Mob.h"

class Input {
    static Vect velocityPlayer;
    static SDL_Event event;
    static SDL_Point mousePos;
    static Mob* selectedMob;
    static bool playerMove;
public:
    static void getInput();
};
