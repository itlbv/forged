#ifndef FORGED_INPUT_H
#define FORGED_INPUT_H

#include "SDL.h"

class Input {
    static SDL_Event event;
public:
    static void getInput();
};


#endif //FORGED_INPUT_H
