#![feature(slice_flatten)]
#![feature(test)]
extern crate test;

const INPUTS: [&str; 3] = ["abcdef", "pqrstuv", "ckczppom"];

fn solution(ip: &'static str) -> usize {
    let mut num = 0usize;

    let mut input: Vec<u8> = ip.bytes().collect();
    let l = input.len();

    loop {
        let mut t = vec![];

        let mut n = num;
        while n != 0 {
            let d = n % 10;
            t.push(d as u8 + b'0');

            n /= 10;
        }

        input.extend(t.into_iter().rev());

        let c = md5_compute(input.clone());

        if c[0..2] == [0, 0] && c[2] & 0b11110000 == 0 {
            return num;
        }

        input.resize(l, 0);

        num += 1;
    }
}

fn md5_compute(mut ip: Vec<u8>) -> [u8; 16] {
    // s specifies the per-round shift amounts
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, // 0..15
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, // 16..31
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, // 32..47
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, // 48..63
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, // 0..3
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, // 4..7
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, // 8..11
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, // 12..15
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, // 16..19
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8, //20..23
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, // 24..27
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, // 28..31
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, // 32..35
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, // 36..39
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, // 40..43
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, // 44..47
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, // 48..51
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1, // 52..55
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, // 56..59
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391, // 60..63
    ];

    let l = ip.len();

    // Padding
    // Since we are only operating on bytes
    // We can append 0x80 (append 1 bit)
    // And then append enough bytes to make it's length equal to 56 % 64
    ip.push(0x80);

    while ip.len() % 64 != 56 {
        ip.push(0x0);
    }

    ip.extend((l * 8).to_le_bytes());

    let input: Vec<u32> = ip
        .chunks(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    let mut state: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

    for chunk in input.chunks(16) {
        let mut lstate = state;

        for i in 0..64 {
            let mut f = 0u32;
            let mut g = i;

            match i {
                0..=15 => {
                    f = (lstate[1] & lstate[2]) | (!lstate[1] & lstate[3]);
                    g = i;
                }
                16..=31 => {
                    f = (lstate[3] & lstate[1]) | (!lstate[3] & lstate[2]);
                    g = (5 * i + 1) % 16;
                }
                32..=47 => {
                    f = lstate[1] ^ lstate[2] ^ lstate[3];
                    g = (3 * i + 5) % 16;
                }
                48..=63 => {
                    f = lstate[2] ^ (lstate[1] | !lstate[3]);
                    g = (7 * i) % 16;
                }

                _ => (),
            }

            f = f + lstate[0] + K[i] + chunk[g];
            lstate.rotate_right(1);
            lstate[1] = lstate[2] + f.rotate_left(S[i]);
        }

        state[0] += lstate[0];
        state[1] += lstate[1];
        state[2] += lstate[2];
        state[3] += lstate[3];
    }

    let mut hash = [0; 16];

    for (i, c) in state.map(|c| c.to_le_bytes()).flatten().iter().enumerate() {
        hash[i] = *c;
    }

    hash
}

fn main() {
    for input in INPUTS.iter() {
        let output = solution(input);
        println!("{output}");
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let result = solution(INPUTS[2]);
        test::black_box(result);
    })
}
