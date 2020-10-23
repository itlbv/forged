#include "Game.h"
#include "Input.h"
#include "Map.h"
#include "Player.h"

bool Game::quit = false;
SDL_Rect Game::viewport{0, 0, 800, 600};
int Game::zoomFactor = 50;
Player Game::player(1, 1);

Game::Game()
        : window(RenderWindow("Forged", 800, 600)) {
    createEntities();
}

void Game::run(unsigned int deltaTime) {
    Input::getInput();

    Map map(window);

    window.updateViewport(&viewport);

    player.update();

    window.startFrame();
    map.render();
    window.render(&player);
    renderEntities();
    window.showFrame();
}

void Game::createEntities() {
    for (int i = 0; i < 5; ++i) {
        int x = rand() % 8;
        int y = rand() % 6;
        Entity e(x, y);
        entities.push_back(e);
    }
}

void Game::renderEntities() {
    for (Entity &e : entities) {
        window.render(&e);
    }
}

