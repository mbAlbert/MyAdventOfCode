use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug)]
struct GardenPlot {
    plant: char,
    area: u32,
    fence_count: u32,
    fence_crosses: HashSet<Cross>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum CrossDirection {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Cross {
    direction: CrossDirection,
    location: (i32, i32),
}

fn get_garden_plot(plot_map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, position: (i32, i32)) -> GardenPlot {
    
    fn update_plot(plot_map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, position: (i32, i32), direction: CrossDirection, plot: &mut GardenPlot, queue: &mut VecDeque<Cross>) {
        let y = position.0;
        let x = position.1;

        // Out of the plot map: we have crossed a fence
        if y < 0 || y == plot_map.len() as i32 || x < 0 || x == plot_map[0].len() as i32 {
            plot.fence_count += 1;
            plot.fence_crosses.insert(Cross { direction, location: (y, x) });
            return;
        }

        // Already been here, update fence count if needed and skip
        if visited[y as usize][x as usize] {
            if plot_map[y as usize][x as usize] != plot.plant {
                plot.fence_count += 1;
                plot.fence_crosses.insert(Cross { direction, location: (y, x) });
            }
            return;
        }

        // Found a different plant: we have crossed a fence
        if plot_map[y as usize][x as usize] != plot.plant {
            plot.fence_count += 1;
            plot.fence_crosses.insert(Cross { direction, location: (y, x) });
            return;
        }

        // Found a another plant in the plot
        visited[y as usize][x as usize] = true;
        plot.area += 1;

        // Push next positions to visit to queue
        queue.push_back(Cross { direction: CrossDirection::North, location: (y - 1, x) });
        queue.push_back(Cross { direction: CrossDirection::South, location: (y + 1, x) });
        queue.push_back(Cross { direction: CrossDirection::West, location: (y, x - 1) });
        queue.push_back(Cross { direction: CrossDirection::East, location: (y, x + 1) });

        // Update plot with next plant findings
        while let Some(fence) = queue.pop_front() {
            update_plot(plot_map, visited, fence.location, fence.direction, plot, queue);
        }

    }

    let mut garden_plot = GardenPlot {
        plant: plot_map[position.0 as usize][position.1 as usize],
        area: 0,
        fence_count: 0,
        fence_crosses: HashSet::new(),
    };
    let mut queue = VecDeque::new();
    update_plot(plot_map, visited, position, CrossDirection::None, &mut garden_plot, &mut queue);
    
    garden_plot
}

fn plot_sides(plot: &GardenPlot) -> u32 {
    let mut sides = 0;
    for cross in &plot.fence_crosses {
        if cross.direction == CrossDirection::North {
            if !plot.fence_crosses.contains(&Cross { direction: CrossDirection::North, location: (cross.location.0, cross.location.1 - 1) }) {
                sides += 1;
            }
        } else if cross.direction == CrossDirection::South {
            if !plot.fence_crosses.contains(&Cross { direction: CrossDirection::South, location: (cross.location.0, cross.location.1 - 1) }) {
                sides += 1;
            }
        } else if cross.direction == CrossDirection::East {
            if !plot.fence_crosses.contains(&Cross { direction: CrossDirection::East, location: (cross.location.0 - 1, cross.location.1) }) {
                sides += 1;
            }
        } else if cross.direction == CrossDirection::West {
            if !plot.fence_crosses.contains(&Cross { direction: CrossDirection::West, location: (cross.location.0 - 1, cross.location.1) }) {
                sides += 1;
            }
        }
    }
    sides
}

pub fn total_price(plot_map: &Vec<Vec<char>>) -> u32 {
    let mut price = 0;
    let mut visited = vec![vec![false; plot_map[0].len()]; plot_map.len()];
    for y in 0..plot_map.len() {
        for x in 0..plot_map[0].len() {
            if !visited[y][x] {
                let plot = get_garden_plot(plot_map, &mut visited, (y as i32, x as i32));
                price += plot.area * plot_sides(&plot);
            }
        }
    }

    price
}
