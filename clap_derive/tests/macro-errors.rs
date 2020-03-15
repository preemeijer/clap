// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed

use std::env;

#[rustversion::attr(any(not(stable), before(1.42)), ignore)]
#[test]
fn ui() {
    let not_present = env::var("CARGO_PKG_NAME").is_err();

    if not_present {
        env::set_var("CARGO_PKG_NAME", "test");
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");

    if not_present {
        env::remove_var("CARGO_PKG_NAME");
    }
}
