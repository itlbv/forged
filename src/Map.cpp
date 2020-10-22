#include "Map.h"
#include "Entity.h"

Map::Map(RenderWindow &window)
        : window(window) {}

void Map::render() {
    int mapSize = Entity::worldToScreen(20);

    //draw horisontal
    for (int i = 0; i < mapSize; ++i) {
        int coord = Entity::worldToScreen(i);
        window.renderLine(0, coord, mapSize, coord);
    }

    //draw vertical
    for (int i = 0; i < mapSize; ++i) {
        int coord = Entity::worldToScreen(i);
        window.renderLine(coord, 0, coord, mapSize);
    }
}
