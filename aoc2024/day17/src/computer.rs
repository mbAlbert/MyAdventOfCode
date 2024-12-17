use std::str::FromStr;

#[derive(Debug)]
pub struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    iptr: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    // pub fn print(&self) {
    //     println!("Register A: {}", self.reg_a);
    //     println!("Register B: {}", self.reg_b);
    //     println!("Register C: {}", self.reg_c);
    //     println!("Instruction Pointer: {}", self.iptr);
    //     println!("Program: {:?}", self.program);
    // }

    fn combo_operand(&self, operand: usize) -> usize {
        if operand <= 3 {
            return operand;
        } else if operand == 4 {
            return self.reg_a;
        } else if operand == 5 {
            return self.reg_b;
        } else if operand == 6 {
            return self.reg_c;
        }

        panic!("Invalid combo operand.");
    }

    fn adv(&mut self, operand: usize) {
        self.reg_a = self.reg_a / (1 << self.combo_operand(operand));
        self.iptr += 2;
    }

    fn bxl(&mut self, operand: usize) {
        self.reg_b = self.reg_b ^ operand;
        self.iptr += 2;
    }

    fn bst(&mut self, operand: usize) {
        self.reg_b = self.combo_operand(operand) % 8;
        self.iptr += 2;
    }

    fn jnz(&mut self, operand: usize) {
        if self.reg_a != 0 {
            self.iptr = operand as usize;
        } else {
            self.iptr += 2;
        }
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
        self.iptr += 2;
    }

    fn out(&mut self, operand: usize) {
        self.output.push((self.combo_operand(operand) % 8) as u8);
        self.iptr += 2;
    }

    fn bdv(&mut self, operand: usize) {
        self.reg_b = self.reg_a / (1 << self.combo_operand(operand));
        self.iptr += 2;
    }

    fn cdv(&mut self, operand: usize) {
        self.reg_c = self.reg_a / (1 << self.combo_operand(operand));
        self.iptr += 2;
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_b = 0;
        self.reg_c = 0;
        self.output.clear();
        self.iptr = 0;
    }

    pub fn set_reg_a(&mut self, a: usize) {
        self.reg_a = a;
    }

    pub fn get_program(&self) -> Vec<u8> {
        self.program.clone()
    }

    pub fn get_output(&self) -> Vec<u8> {
        self.output.clone()
    }

    pub fn run_program(&mut self) {
        while self.iptr < self.program.len() {
            let opcode = self.program[self.iptr];
            let operand = self.program[self.iptr + 1] as usize;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Invalid opcode."),
            };
        }
    }

}

impl FromStr for Computer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let reg_a = lines.next().unwrap().trim().strip_prefix("Register A: ").unwrap().parse::<usize>().unwrap();
        let reg_b = lines.next().unwrap().trim().strip_prefix("Register B: ").unwrap().parse::<usize>().unwrap();
        let reg_c = lines.next().unwrap().trim().strip_prefix("Register C: ").unwrap().parse::<usize>().unwrap();

        lines.next();

        let program: Vec<_> = lines.next().unwrap()
            .trim()
            .strip_prefix("Program: ").unwrap()
            .split(",")
            .map(|digit| {
                digit.parse::<u8>().unwrap()
            })
            .collect();

        Ok ({
            Computer {
                reg_a, reg_b, reg_c, iptr: 0, program, output: Vec::new()
            }
        })
    }
}