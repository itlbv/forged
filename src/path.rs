use std::collections::{BinaryHeap, HashMap};
use sdl2::libc::newlocale;
use crate::main;
use crate::noise_generator::Noise;

pub fn dijkstra(map: &Noise, start: (usize, usize), finish: (usize, usize))
                -> Vec<(usize, usize)> {
    let mut frontier = vec![];
    frontier.push(start);
    let mut came_from: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    came_from.insert(start, None);
    let mut cost_so_far: HashMap<(usize, usize), f64> = HashMap::new();
    cost_so_far.insert(start, 0.0);

    while !frontier.is_empty() {
        sort_by_value(&mut frontier, map);
        let current = frontier.remove(0);
        if current == finish { break; }
        let neighbors = borrow_orthogonal_neighbours(map, current);
        for neighbor in neighbors {
            if cost_so_far.contains_key(&neighbor) { continue; }
            let new_cost = cost_so_far[&current] + map.get_value(neighbor.0, neighbor.1);
            if !cost_so_far.contains_key(&neighbor)
                || new_cost < cost_so_far[&neighbor] {
                cost_so_far.insert(neighbor, new_cost);
                came_from.insert(neighbor, Option::Some(current));
                frontier.push(neighbor);
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

fn sort_by_value(vec: &mut Vec<(usize, usize)>, map: &Noise) {
    let mut min_value = 99.0;

    let mut i = 0;
    loop {
        if i >= vec.len() { break; }

        let new_value = map.get_value(vec[i].0, vec[i].1);
        if new_value < min_value {
            min_value = new_value;
            let el = vec.swap_remove(i);
            vec.insert(0, el);

            continue;
        }

        i += 1;
    }
}

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