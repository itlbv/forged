#include <iostream>
#include <SDL.h>

#include "RenderWindow.h"

int main(int argc, char **argv)
{
    if (SDL_Init(SDL_INIT_VIDEO) > 0)
        std::cout << "SDL_Init FAILED. Error: " << SDL_GetError() << std::endl;

    RenderWindow renderWindow("Forged", 800, 600);
    renderWindow.render();

    SDL_Delay(2000);
    renderWindow.cleanUp();
    SDL_Quit();

    return 0;
}
