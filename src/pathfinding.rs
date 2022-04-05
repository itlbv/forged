use std::collections::HashMap;
use crate::noise_generator::Noise;

pub fn breadth_first(map: &Noise, start: (usize, usize), finish: (usize, usize))
                     -> Vec<(usize, usize)> {
    let mut frontier = vec![(start.0, start.1)];
    let mut came_from: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    came_from.insert((start.0, start.1), None);

    while !frontier.is_empty() {
        let current = frontier.remove(0);
        if current == finish { break; }
        let neighbors = borrow_orthogonal_neighbours(map, current);
        for neighbor in neighbors {
            if !came_from.contains_key(&neighbor) {
                frontier.push(neighbor);
                came_from.insert(neighbor, Option::Some(current));
            }
        }
    }

    let mut path = vec![];
    let mut current = finish;
    while current != start {
        path.push(current);
        current = came_from[&current].unwrap();
    };
    path
}

fn borrow_orthogonal_neighbours(map: &Noise, node: (usize, usize)) -> Vec<(usize, usize)> {
    let (width, height) = map.size();
    let mut neighbours = vec![];
    if node.0 > 0 { neighbours.push((node.0 - 1, node.1)); }
    if node.0 < width - 1 { neighbours.push((node.0 + 1, node.1)); }
    if node.1 > 0 { neighbours.push((node.0, node.1 - 1)); }
    if node.1 < height - 1 { neighbours.push((node.0, node.1 + 1)); }

    neighbours
}