#pragma once

#include <SDL.h>
#include "Vect.h"

class Input {
    static Vect velocityPlayer;
    static SDL_Event event;
public:
    static void getInput();
};
