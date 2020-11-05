#include "Game.h"
#include "Input.h"
#include "Map.h"
#include "Mob.h"

bool Game::quit = false;
SDL_Rect Game::viewport{0, 0, 800, 600};
int Game::zoomFactor = 50;
std::vector<Mob> Game::mobs;             // TODO make it a vector of pointers?
Player Game::player(1, 1);
double Game::delta = 0;

Game::Game()
        : window(RenderWindow("Forged", 800, 600)) {
    createMobs();
}

void Game::run(unsigned int deltaTime) {
    delta = (double)deltaTime / 1000;

    Input::getInput();

    Map map(window);

    window.updateViewport(&viewport);

    player.update();

    updateMobs();

    window.startFrame();
    map.render();
    window.renderEntity(player);
    renderEntities();
    window.showFrame();
}

void Game::createMobs() {
    for (int i = 0; i < 5; ++i) {
        int x = rand() % 8;
        int y = rand() % 6;
        Mob m(x, y);
        mobs.push_back(m);
    }
}

void Game::updateMobs() {
    for (Mob &m : mobs) {
        m.update();
    }
}

void Game::renderEntities() {
    for (Entity &e : mobs) {
        window.renderEntity(e);
    }
}

