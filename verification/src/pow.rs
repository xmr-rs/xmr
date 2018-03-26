// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use primitives::H256;
use std::u64;

#[derive(Debug)]
pub struct Difficulty(pub u64);

pub fn is_valid_proof_of_work(hash: H256, Difficulty(difficulty): Difficulty) -> bool {
    let comps = hash.u64_components();

    let (top, high0) = mul(comps.3, difficulty);
    if high0 != 0 {
        return false;
    }
    let (_low1, high1) = mul(comps.0, difficulty);
    let (low2, high2) = mul(comps.1, difficulty);
    let (low3, high3) = mul(comps.2, difficulty);

    !cadc(high3, top, cadc(high2, low3, cadd(high1, low2)))
}

fn cadd(a: u64, b: u64) -> bool {
    a + b < a
}

fn cadc(a: u64, b: u64, c: bool) -> bool {
    a + b < a || (c && a + b == u64::MAX)
}

fn mul(a: u64, b: u64) -> (u64, u64) {
    let alow = a & 0xffffffff;
    let ahigh = a >> 32;
    let blow = b & 0xffffffff;
    let bhigh = b >> 32;

    let res = alow * blow;
    let lowres1 = res & 0xffffffff;
    let carry = res >> 32;

    let res = ahigh * blow + carry;
    let highreshigh1 = res >> 32;
    let highreslow1 = res & 0xffffffff;

    let res = alow * bhigh;
    let lowres2 = res & 0xffffffff;
    let carry = res >> 32;

    let res = ahigh * bhigh + carry;
    let highreshigh2 = res >> 32;
    let highreslow2 = res & 0xffffffff;

    let r = highreslow1 + lowres2;
    let carry = r >> 32;
    let low = (r << 32) | lowres1;
    let r = highreshigh1 + highreslow2 + carry;
    let d3 = r & 0xffffffff;
    let carry = r >> 32;
    let r = highreshigh2 + carry;
    let high = d3 | (r << 32);

    (low, high)
}
