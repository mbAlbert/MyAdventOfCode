use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
pub struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let metrics = s.trim().split_whitespace()
            .map(|part| {
                part.split_once("=").unwrap().1
                    .split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .tuple_windows()
            })
            .flatten()
            .take(2)
            .collect::<Vec<_>>();

        Ok(Robot {
            position: metrics[0],
            velocity: metrics[1], 
        })
    }
}