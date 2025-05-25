use std::collections::VecDeque;

#[derive(Debug)]
struct GardenPlot {
    plant: char,
    area: u32,
    fences: u32,
}

fn get_garden_plot(plot_map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, position: (i32, i32)) -> GardenPlot {
    
    fn update_plot(plot_map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, position: (i32, i32), plot: &mut GardenPlot, queue: &mut VecDeque<(i32, i32)>) {
        let y = position.0;
        let x = position.1;

        // Out of the plot map: we have crossed a fence
        if y < 0 || y == plot_map.len() as i32 || x < 0 || x == plot_map[0].len() as i32 {
            plot.fences += 1;
            return;
        }

        // Already been here, update fence count if needed and skip
        if visited[y as usize][x as usize] {
            if plot_map[y as usize][x as usize] != plot.plant {
                plot.fences += 1;
            }
            return;
        }

        // Found a different plant: we have crossed a fence
        if plot_map[y as usize][x as usize] != plot.plant {
            plot.fences += 1;
            return;
        }

        // Found a another plant in the plot
        visited[y as usize][x as usize] = true;
        plot.area += 1;

        // Push next positions to visit to queue
        queue.push_back((y - 1, x));
        queue.push_back((y + 1, x));
        queue.push_back((y, x - 1));
        queue.push_back((y, x + 1));

        // Update plot with next plant findings
        while let Some(next_position) = queue.pop_front() {
            update_plot(plot_map, visited, next_position, plot, queue);
        }

    }

    let mut garden_plot = GardenPlot {
        plant: plot_map[position.0 as usize][position.1 as usize],
        area: 0,
        fences: 0,
    };
    let mut queue = VecDeque::new();
    update_plot(plot_map, visited, position, &mut garden_plot, &mut queue);
    
    garden_plot
}

pub fn total_price(plot_map: &Vec<Vec<char>>) -> u32 {
    let mut price = 0;
    let mut visited = vec![vec![false; plot_map[0].len()]; plot_map.len()];
    for y in 0..plot_map.len() {
        for x in 0..plot_map[0].len() {
            if !visited[y][x] {
                let plot = get_garden_plot(plot_map, &mut visited, (y as i32, x as i32));
                // println!("{:?}", plot);
                price += plot.area * plot.fences;
            }
        }
    }

    price
}
