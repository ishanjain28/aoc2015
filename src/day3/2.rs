#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn parse(input: &'static str) -> Vec<char> {
    input.trim().chars().collect()
}

fn solution(input: Vec<char>) -> usize {
    let mut set = HashSet::new();
    let (mut sx, mut sy) = (0, 0);
    let (mut rsx, mut rsy) = (0, 0);

    set.insert((sx, sy));

    for (i, c) in input.into_iter().enumerate() {
        match (i % 2, c) {
            (1, 'v') => rsx += 1,
            (1, '>') => rsy += 1,
            (1, '^') => rsx -= 1,
            (1, '<') => rsy -= 1,
            (0, 'v') => sx += 1,
            (0, '>') => sy += 1,
            (0, '^') => sx -= 1,
            (0, '<') => sy -= 1,

            _ => (),
        }

        if i % 2 == 0 {
            set.insert((sx, sy));
        } else {
            set.insert((rsx, rsy));
        }
    }

    set.len()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let output = solution(output);
        println!("{output}");
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
