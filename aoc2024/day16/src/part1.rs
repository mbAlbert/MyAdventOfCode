use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    dir: (isize, isize),
    score: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse compare order for Min heap
        other.score.cmp(&self.score) 
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn maze_lowest_score(maze: &Vec<Vec<char>>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    // Push initial position into heap
    let start_node = find_reindeer(maze).unwrap();
    heap.push( start_node );
    
    // Run algorithm for each remaining position
    while let Some( curr_node ) = heap.pop() {
        let (y, x) = curr_node.pos;
        let (dy, dx) = curr_node.dir;

        // Target is reached
        if maze[y][x] == 'E' {
            return Some(curr_node.score);
        }

        // Insert new position
        seen.insert((curr_node.pos, curr_node.dir));

        // Next nodes
        let next_nodes = [
            Node { pos: ((y as isize + dy) as usize, (x as isize + dx) as usize), dir: (dy, dx), score: curr_node.score + 1 }, // Same direction
            Node { pos: (y, x), dir: (dx, -dy), score: curr_node.score + 1000 }, // Clockwise turn
            Node { pos: (y, x), dir: (-dx, dy), score: curr_node.score + 1000 }, // Counterclockwise turn
        ];

        for node in next_nodes {
            if maze[node.pos.0][node.pos.1] == '#' {
                continue;
            }

            if seen.contains(&(node.pos, node.dir)) {
                continue;
            }

            heap.push(node);
        }
    }

    None
}

fn find_reindeer(maze: &Vec<Vec<char>>) -> Option<Node> {
    for r in 0..maze.len() {
        for c in 0..maze[0].len() {
            if maze[r][c] == 'S' {
                return Some(Node { pos: (r, c), dir: (0, 1), score: 0 });
            }
        }
    }

    None
}

pub fn result(maze: &Vec<Vec<char>>) -> usize {
    maze_lowest_score(maze).unwrap()
} 
