use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}
impl Instruction {
    fn execute(&self, cycle: u64, value: &mut i64) -> bool {
        match *self {
            Self::Noop => true,
            Self::Addx(x) => {
                if cycle == 1 {
                    *value += x;
                    true
                } else {
                    false
                }
            }
        }
    }
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::Addx(parts[1].parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Cpu {
    value: i64,
    program: Vec<Instruction>,
    inst: usize,
    inst_cycle: u64,
    cycle: u64,
}
impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            value: 1,
            program,
            inst: 0,
            inst_cycle: 0,
            cycle: 0,
        }
    }

    fn step(&mut self) -> bool {
        let inst = &self.program[self.inst];
        let next = inst.execute(self.inst_cycle, &mut self.value);
        if next {
            self.inst += 1;
            self.inst_cycle = 0;
            if self.inst >= self.program.len() {
                return false;
            }
        } else {
            self.inst_cycle += 1;
        }
        self.cycle += 1;
        true
    }

    fn run_to(&mut self, cycle: u64) -> bool {
        while self.cycle < cycle {
            if !self.step() {
                return false;
            }
        }
        true
    }
}

pub fn run_a(input: &str) {
    let mut cpu = Cpu::new(
        input
            .lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect(),
    );

    let mut cycle = 19;
    let mut sum = 0;
    loop {
        if !cpu.run_to(cycle) {
            break;
        }
        sum += (cycle + 1) as i64 * cpu.value;
        cycle += 40;
    }

    println!("Sum: {sum}");
}

pub fn run_b(input: &str) {
    let mut cpu = Cpu::new(
        input
            .lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect(),
    );

    let mut x_pos = 0;
    for y in 0..6 {
        for _ in 0..40 {
            if (x_pos - cpu.value).abs() <= 1 {
                print!("#");
            } else {
                print!(" ");
            }
            cpu.step();
            x_pos += 1;
        }
        x_pos = 0;
        println!();
    }
}

