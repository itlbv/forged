#include "Game.h"
#include "Input.h"
#include <random>

bool Game::quit = false;

Game::Game()
        : window(RenderWindow("Forged", 800, 600)),
          player(Entity(10, 10)) {
    createEntities();
}

void Game::run(unsigned int deltaTime) {
    Input::getInput();

    window.startFrame();
    window.render(&player);
    renderEntities();
    window.showFrame();
}

void Game::createEntities() {
    for (int i = 0; i < 5; ++i) {
        int x = rand() % 800;
        int y = rand() % 600;
        Entity e(x, y);
        entities.push_back(e);
    }
}

void Game::renderEntities() {
    for (Entity &e : entities) {
        window.render(&e);
    }
}

