use crc32fast::Hasher;
use ring::digest::Context;
use ring::digest::SHA256;
use ring::test;
use std::fs;
use std::time::Instant;

// orig crc32 = 450303355
//orig sha256 = 3008841bd2c7967ecce978594d72705a6be6ed073a2a3bfe2a9fa08337bdec44

// struct Checkpoints {
//     _5: Hasher,
//     _10: Hasher,
//     _15: Hasher,
//     _20: Hasher,
//     _25: Hasher,
//     _30: Hasher,
//     _35: Hasher,
//     _40: Hasher,
//     _45: Hasher,
//     _50: Hasher,
//     _55: Hasher,
//     _60: Hasher,
//     _65: Hasher,
//     _70: Hasher,
//     _75: Hasher,
//     _80: Hasher,
//     _85: Hasher,
//     _90: Hasher,
//     _95: Hasher
// }

fn crc32(data: &mut [u8], correct_checksum: u32) {
    for i in 0..data.len() {
        let mut h = Hasher::new();
        h.update(&data[0..i]);
        let last_hash = h.finalize();

        for p in 0..=255 {
            let mut h = Hasher::new_with_initial(last_hash);
            h.update(&[p]);
            h.update(&data[i + 1..]);

            if correct_checksum == h.finalize() {
                data[i] = p;
                return;
            }
        }
    }
}

fn sha256(data: &mut [u8], correct_checksum: String) {
    // add check for valid 256 hex, maybe break into seprate func

    let correct_checksum = test::from_hex(correct_checksum.as_str()).unwrap();

    for i in 0..data.len() {
        let mut last_ctx = Context::new(&SHA256);
        last_ctx.update(&data[0..i]);

        for p in 0..=255 {
            let mut ctx = last_ctx.clone();
            ctx.update(&[p]);
            ctx.update(&data[i + 1..]);

            if correct_checksum == ctx.finish().as_ref() {
                data[i] = p;
                return;
            }
        }
    }
}

fn main() {
    let mut data = fs::read("./test_line_31.txt").expect("couldn't open file");
    let correct_checksum = 450303355;

    let now = Instant::now();
    // crc32(&mut data, correct_checksum);
    sha256(
        &mut data,
        "3008841bd2c7967ecce978594d72705a6be6ed073a2a3bfe2a9fa08337bdec44".to_string(),
    );
    println!("Elapsed: {:.2?}", now.elapsed());

    fs::write("output_test.txt", data).expect("couldn't write output file");
}
