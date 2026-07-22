//! Guards against exponential backtracking in the expression grammar.
//!
//! `conditional_expression` and `assignment_expression` used to be written as a
//! `choice` whose alternatives shared a common prefix, so a non-ternary,
//! non-assignment expression was re-parsed 3x per funnel. `sizeof((T[]){ ... })`
//! passes through two such funnels per nesting level -- the parenthesized
//! expression and the braced initializer -- which made parse time grow ~9x per
//! level: depth 7 took 271s.

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use cgrammar::*;

/// `sizeof((int[]){ sizeof((int[]){ ... }) })`, nested `depth` deep.
fn nested_sizeof(depth: usize) -> String {
    let mut expr = String::from("0");
    for _ in 0..depth {
        expr = format!("sizeof((int[]){{ {expr} }})");
    }
    format!("int x = {expr};\n")
}

#[test]
fn nested_compound_literal_sizeof_is_not_exponential() {
    // At 9x/level the old parser needs ~9^16 times the depth-0 cost here, so a
    // regression blows the budget rather than merely being slow. Left-factored,
    // this is ~130 tokens and parses in milliseconds.
    const DEPTH: usize = 16;
    const BUDGET: Duration = Duration::from_secs(10);

    let source = nested_sizeof(DEPTH);

    // Parse off-thread so a regression fails on the timeout instead of hanging
    // the test binary. The recursive-descent parser wants a deep stack.
    let (tx, rx) = mpsc::channel();
    thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let start = Instant::now();
            let (tokens, _) = lex(&source, None);
            let result = translation_unit().parse(tokens.as_input());
            let _ = tx.send((result.has_errors(), start.elapsed()));
        })
        .unwrap();

    match rx.recv_timeout(BUDGET) {
        Ok((has_errors, elapsed)) => {
            assert!(!has_errors, "depth {DEPTH} failed to parse");
            assert!(elapsed < BUDGET, "depth {DEPTH} took {elapsed:?}");
        }
        Err(_) => panic!("depth {DEPTH} did not parse within {BUDGET:?}: exponential backtracking"),
    }
}
