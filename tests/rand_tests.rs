// Copyright 2015-2019 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use ring::{
    rand::{self, SecureRandom as _},
    test,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn test_system_random_lengths() {
    const LINUX_LIMIT: usize = 256;
    const WEB_LIMIT: usize = 65536;

    // Test that `fill` succeeds for various interesting lengths. `256` and
    // multiples thereof are interesting because that's an edge case for
    // `getrandom` on Linux.
    let lengths = [
        0,
        1,
        2,
        3,
        96,
        LINUX_LIMIT - 1,
        LINUX_LIMIT,
        LINUX_LIMIT + 1,
        LINUX_LIMIT * 2,
        511,
        512,
        513,
        4096,
        WEB_LIMIT - 1,
        WEB_LIMIT,
        WEB_LIMIT + 1,
        WEB_LIMIT * 2,
    ];

    for len in lengths.iter() {
        let mut buf = vec![0; *len];

        let rng = rand::SystemRandom::new();
        assert!(rng.fill(&mut buf).is_ok());

        // If `len` < 96 then there's a big chance of false positives, but
        // otherwise the likelihood of a false positive is so too low to
        // worry about.
        if *len >= 96 {
            assert!(buf.iter().any(|x| *x != 0));
        }
    }
}

#[test]
fn test_randomly_constructable() {
    // Picked arbitrarily
    const BY: u8 = 0x5A;

    let rng = test::rand::FixedByteRandom {
        byte: BY
    };

    assert!(rand::generate::<[u8; 0]>(&rng).is_ok());
    assert!(rand::generate::<[u64; 0]>(&rng).is_ok());
    assert!(rand::generate::<[usize; 0]>(&rng).is_ok());

    let arr: [u8; 256] = rand::generate(&rng)
        .unwrap()
        .expose();
    assert!(arr.iter().all(|&x| x == BY));

    let arr: [u32; 256] = rand::generate(&rng)
        .unwrap()
        .expose();
    // Endianness doesn't matter since all the bytes are the same
    assert!(arr.iter().all(|&x| x.to_le_bytes() == [BY; 4]));
}

#[test]
fn test_system_random_traits() {
    test::compile_time_assert_clone::<rand::SystemRandom>();
    test::compile_time_assert_send::<rand::SystemRandom>();

    assert_eq!(
        "SystemRandom(())",
        format!("{:?}", rand::SystemRandom::new())
    );
}
