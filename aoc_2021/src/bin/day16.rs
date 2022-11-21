use std::slice::Iter;

trait BitReader {
    fn bits_to_num(&mut self, count: usize) -> u64;
}

impl BitReader for Iter<'_, u8> {
    fn bits_to_num(&mut self, count: usize) -> u64 {
        self.take(count).fold(0, |acc, bit| (acc << 1) | *bit as u64)
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let decode = decode_hex(&lines[0]);
    let (version, values) = calculate(&decode);
    println!("Part 1: {}", version);
    println!("Part 2: {}", values[0]);
}

fn decode_hex(hex: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for c in hex.chars() {
        let val = if c >= 'A' {
            c as u8 - b'A' + 10
        } else {
            c as u8 - b'0'
        };
        for b in [8, 4, 2, 1] {
            result.push(((val & b) != 0) as u8);
        }
    }

    result
}

fn calculate(bits: &[u8]) -> (u64, Vec<u64>) {
    let mut iter = bits.iter();
    calculate_packets(&mut iter).unwrap()
}

fn calculate_packets(iter: &mut Iter<'_, u8>) -> Option<(u64, Vec<u64>)> {
    let mut version = 0;
    let mut value = Vec::new();
    let mut decoded = false;

    if let Some(v) = iter.next() {
        decoded = true;
        let rest = iter.bits_to_num(2);
        version += (v << 2) as u64 | rest;
        let msg_type = iter.bits_to_num(3);
        if msg_type == 4 {
            let mut digit = iter.bits_to_num(5);
            let mut result = 0;
            while digit > 15 {
                result = result << 4 | (digit & 0xf);
                digit = iter.bits_to_num(5);
            }
            result = result << 4 | (digit & 0xf);
            value.push(result);
        } else {
            let len_type = iter.bits_to_num(1);
            if len_type == 0 {
                let subpackets_len = iter.bits_to_num(15);
                let subpackets: Vec<u8> = iter.take(subpackets_len as usize).copied().collect();
                let mut newiter = subpackets.iter();
                while let Some((versum, valsum)) = calculate_packets(&mut newiter) {
                    version += versum;
                    value.extend(valsum);
                }
            } else {
                let subpackets_count = iter.bits_to_num(11);
                for _ in 0..subpackets_count {
                    let (versum, valsum) = calculate_packets(iter).unwrap();
                    version += versum;
                    value.extend(valsum);
                }
            }
            match msg_type {
                0 => {
                    let newval = value.iter().sum();
                    value.clear();
                    value.push(newval);
                }
                1 => {
                    let newval = value.iter().product();
                    value.clear();
                    value.push(newval);
                }
                2 => {
                    let newval = *value.iter().min().unwrap();
                    value.clear();
                    value.push(newval);
                }
                3 => {
                    let newval = *value.iter().max().unwrap();
                    value.clear();
                    value.push(newval);
                }

                5 => {
                    let left = value[0];
                    let right = value[1];
                    value.clear();
                    value.push((left > right) as u64);
                }

                6 => {
                    let left = value[0];
                    let right = value[1];
                    value.clear();
                    value.push((left < right) as u64);
                }

                7 => {
                    let left = value[0];
                    let right = value[1];
                    value.clear();
                    value.push((left == right) as u64);
                }

                _ => unreachable!(),
            }
        }
    }
    if decoded {
        Some((version, value))
    } else {
        None
    }
}
