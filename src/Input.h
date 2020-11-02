#pragma once

#include <SDL.h>
#include "Vect.h"
#include "Entity.h"

class Input {
    static Vect velocityPlayer;
    static SDL_Event event;
    static SDL_Point mousePos;
    static Entity* selectedEntity;
    static bool playerMove;
public:
    static void getInput();
};
