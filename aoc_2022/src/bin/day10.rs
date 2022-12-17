struct Circuit {
    cycles: i32,
    value: i32,
    signal_strength: i32,
    crt: Vec<char>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            cycles: 0,
            value: 1,
            signal_strength: 0,
            crt: Vec::with_capacity(240),
        }
    }

    fn add_cycle(&mut self) {
        self.draw_crt();
        self.cycles += 1;
        if let Some(strength) = self.check_signal_strength() {
            self.signal_strength += strength;
        }
    }

    fn check_signal_strength(&self) -> Option<i32> {
        match self.cycles {
            20 | 60 | 100 | 140 | 180 | 220 => Some(self.cycles * self.value),
            _ => None,
        }
    }

    fn draw_crt(&mut self) {
        let screen = self.cycles % 40;
        let first = self.value - 1;
        let last = self.value + 1;
        if (first..=last).contains(&screen) {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }
    }

    fn draw(&self) {
        for (i, p) in self.crt.iter().enumerate() {
            if i % 40 == 0 {
                println!();
            }
            print!("{}", p);
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();

    let mut circuit = Circuit::new();

    for line in input.lines() {
        if let Some((_, val)) = line.split_once(' ') {
            circuit.add_cycle();
            circuit.add_cycle();
            circuit.value += val.parse::<i32>().unwrap();
        } else {
            circuit.add_cycle();
        }
    }

    println!("Part 1: {}", circuit.signal_strength);
    println!("Part 2: ");
    circuit.draw();
}
