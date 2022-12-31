#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn parse(input: &'static str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.splitn(3, 'x')
                .map(|num| num.bytes().fold(0, |a, x| (a * 10) + (x - b'0') as i64))
                .collect()
        })
        .collect()
}

fn solution(input: Vec<Vec<i64>>) -> i64 {
    input.into_iter().fold(0, |a, c| {
        let s1 = 2 * (c[0] + c[1]);
        let s2 = 2 * (c[1] + c[2]);
        let s3 = 2 * (c[2] + c[0]);

        a + c[0] * c[1] * c[2] + s1.min(s2.min(s3))
    })
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
