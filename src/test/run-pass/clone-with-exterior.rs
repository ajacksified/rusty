// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pretty-expanded FIXME #23616

#![allow(unknown_features)]
#![feature(box_syntax, std_misc)]

use std::thread::Thread;

struct Pair {
    a: int,
    b: int
}

pub fn main() {
    let z: Box<_> = box Pair { a : 10, b : 12};

    let _t = Thread::spawn(move|| {
        assert_eq!(z.a, 10);
        assert_eq!(z.b, 12);
    });
}
