fn main() {
    // Part 1
    let safe_reports_p1 = include_str!("input.txt")
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|report| {
            let decreasing = report.windows(2).all(|pair| (pair[0] > pair[1]) && ((pair[0]-pair[1]).abs() <= 3) );
            let increasing = report.windows(2).all(|pair| (pair[0] < pair[1]) && ((pair[0]-pair[1]).abs() <= 3) );
            decreasing || increasing
        })
        .count();

    println!("Part 1. Safe reports: {}", safe_reports_p1);

    // Part 2
    let safe_reports_p2 = include_str!("input.txt")
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                let decreasing = report_copy.windows(2).all(|pair| (pair[0] > pair[1]) && ((pair[0]-pair[1]).abs() <= 3) );
                let increasing = report_copy.windows(2).all(|pair| (pair[0] < pair[1]) && ((pair[0]-pair[1]).abs() <= 3) );
                decreasing || increasing
            })
        })
        .count();

        println!("Part 2. Safe reports: {}", safe_reports_p2);

}
