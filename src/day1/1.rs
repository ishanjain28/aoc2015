#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn parse(input: &'static str) -> Vec<char> {
    input.trim().chars().collect()
}

fn solution(input: Vec<char>) -> i64 {
    let mut answer = 0;

    for c in input {
        match c {
            '(' => answer += 1,
            ')' => answer -= 1,
            _ => unreachable!(),
        }
    }

    answer
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let output = solution(output);
        println!("{output}");
    }
}

#[test]
fn solution_test() {
    assert_eq!(solution("(())".chars().collect()), 0);
    assert_eq!(solution("()()".chars().collect()), 0);
    assert_eq!(solution("(((".chars().collect()), 3);
    assert_eq!(solution("(()(()(".chars().collect()), 3);
    assert_eq!(solution("))(((((".chars().collect()), 3);
    assert_eq!(solution("())".chars().collect()), -1);
    assert_eq!(solution(")())())".chars().collect()), -3);
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
