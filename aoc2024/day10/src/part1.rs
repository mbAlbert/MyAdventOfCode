use std::collections::VecDeque;
use std::collections::HashSet;

pub fn trailhead_score(topographic_map: &Vec<Vec<u32>>) -> u32
{
    fn bfs(map: &Vec<Vec<u32>>, height: u32, limit: u32, position: (u32, u32), offsets: &Vec<(i32, i32)>, queue: &mut VecDeque<(u32, (u32, u32))>, set: &mut HashSet<(u32, u32)>)
    {
        if height == limit {
            set.insert(position);
            return;
        }

        let y = position.0 as i32;
        let x = position.1 as i32;

        for (dy, dx) in offsets {
            if (y + dy) >= 0 && (y + dy) < map.len() as i32 
                && (x + dx) >= 0 && (x + dx) < map[0].len() as i32 {
                    if map[(y + dy) as usize][(x + dx) as usize] == height + 1 {
                        queue.push_back( (height + 1, ((y + dy) as u32,(x + dx) as u32)) );
                    }
            } 
        }

        // println!("{:?}", queue);

        while let Some( (next_height, next_position) ) = queue.pop_front() {
            bfs(map, next_height, limit, next_position, offsets, queue, set);
        }
    }


    let offsets = vec![
        (-1, 0), (0, -1), (0, 1), (1, 0),
    ];
    let mut queue = VecDeque::new();
    let mut score = 0;
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[0].len() {
            if topographic_map[y][x] == 0 {
                let mut score_set = HashSet::new();
                bfs(topographic_map, 0, 9, (y as u32, x as u32), &offsets, &mut queue, &mut score_set);
                // println!("y: {} x: {} score: {}", y, x, score_set.len());
                score += score_set.len();
            }
        }
    }

    score as u32
}
