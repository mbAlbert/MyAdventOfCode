use std::str::FromStr;

#[derive(Debug)]
pub struct ClawMachine {
    pub button_a: (i64, i64),
    pub button_b: (i64, i64),
    pub prize: (i64, i64),
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Button A
        let ba = lines.next().unwrap();
        let ba = ba.trim().strip_prefix("Button A: ").unwrap();
        let xy: Vec<_> = ba.split(", ").collect();
        let x = xy[0].strip_prefix("X+").unwrap().parse::<i64>().unwrap();
        let y = xy[1].strip_prefix("Y+").unwrap().parse::<i64>().unwrap();
        let button_a = (x, y);

        // Button B
        let bb = lines.next().unwrap();
        let bb = bb.trim().strip_prefix("Button B: ").unwrap();
        let xy: Vec<_> = bb.split(", ").collect();
        let x = xy[0].strip_prefix("X+").unwrap().parse::<i64>().unwrap();
        let y = xy[1].strip_prefix("Y+").unwrap().parse::<i64>().unwrap();
        let button_b = (x, y);
        
        // Prize
        let prize = lines.next().unwrap();
        let prize = prize.trim().strip_prefix("Prize: ").unwrap();
        let xy: Vec<_> = prize.split(", ").collect();
        let x = xy[0].strip_prefix("X=").unwrap().parse::<i64>().unwrap();
        let y = xy[1].strip_prefix("Y=").unwrap().parse::<i64>().unwrap();
        let prize = (x, y);

        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }
}