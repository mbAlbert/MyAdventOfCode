use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn unwind(
    end_nodes: &HashSet<((usize, usize), (isize, isize))>, 
    backtrack: &HashMap<((usize, usize), (isize, isize)), HashSet<((usize, usize), (isize, isize))>>
) -> HashSet<(usize, usize)> {
    let mut seen_nodes = end_nodes.clone();
    let mut seen_positions = seen_nodes.iter().fold(HashSet::new(), |mut set, node| {
        set.insert(node.0);
        set
    });
    let mut backtrack_queue = VecDeque::from_iter(end_nodes);
    
    while let Some(key) = backtrack_queue.pop_front() {
        if let Some(set) = backtrack.get(&key) {
            for node in set {
                if seen_nodes.contains(node) {
                    continue;
                }
                seen_nodes.insert(*node);
                seen_positions.insert(node.0);
                backtrack_queue.push_back(node);
            }
        }
    }
    seen_positions
}

fn maze_lowest_score(maze: &Vec<Vec<char>>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut node_best_score_map = HashMap::new();
    let mut node_backtrack_map = HashMap::new();
    let mut end_nodes = HashSet::new();
    let mut best_score = usize::MAX;

    // Push initial position into heap
    let start_node = find_reindeer(maze).unwrap();
    heap.push( start_node );
    
    // Run algorithm for each remaining position
    while let Some( curr_node ) = heap.pop() {
        let (y, x) = curr_node.pos;
        let (dy, dx) = curr_node.dir;

        if maze[y][x] == 'E' {
            if curr_node.score > best_score {
                break;
            }
            best_score = curr_node.score;
            end_nodes.insert((curr_node.pos, curr_node.dir));
        }

        if let Some(node_best_score) = node_best_score_map.get(&(curr_node.pos, curr_node.dir)) {
            if curr_node.score > *node_best_score {
                continue;
            }
        }
        node_best_score_map.entry((curr_node.pos, curr_node.dir)).or_insert(curr_node.score);

        let next_nodes = [
            Node { pos: ((y as isize + dy) as usize, (x as isize + dx) as usize), dir: (dy, dx), score: curr_node.score + 1 }, // Same direction
            Node { pos: (y, x), dir: (dx, -dy), score: curr_node.score + 1000 }, // Clockwise turn
            Node { pos: (y, x), dir: (-dx, dy), score: curr_node.score + 1000 }, // Counterclockwise turn
        ];

        for node in next_nodes {
            if maze[node.pos.0][node.pos.1] == '#' {
                continue;
            }

            if let Some(node_best_score) = node_best_score_map.get(&(node.pos, node.dir)) {
                if node.score > *node_best_score {
                    continue;
                }
                node_backtrack_map.remove_entry(&(node.pos, node.dir));
            }
            node_backtrack_map.entry((node.pos, node.dir)).or_insert_with(HashSet::new).insert((curr_node.pos, curr_node.dir));
            heap.push(node);
        }
    }

    Some(unwind(&end_nodes, &node_backtrack_map).len())
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
