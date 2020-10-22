#include "Game.h"
#include "Input.h"
#include "Map.h"
#include <random>

bool Game::quit = false;

Game::Game()
        : window(RenderWindow("Forged", 800, 600)),
          player(Entity(1, 1)) {
    createEntities();
}

void Game::run(unsigned int deltaTime) {
    Input::getInput();

    Map map(window);

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

