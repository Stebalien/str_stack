#![feature(test)]
extern crate test;
extern crate str_stack;

use str_stack::StrStack;
use std::fmt::Write;

#[bench]
fn bench_index(b: &mut test::Bencher) {
    let mut stack = StrStack::new();
    for i in 0..100 {
        write!(stack, "{}", i);
    }

    b.iter(|| {
        for i in 0..stack.len() {
            test::black_box(&stack[test::black_box(i)]);
        }
    })
}

#[bench]
fn bench_index_naive(b: &mut test::Bencher) {
    let stack: Vec<_> = (0..100).map(|i| i.to_string()).collect();

    b.iter(|| {
        for i in 0..stack.len() {
            test::black_box(&stack[test::black_box(i)]);
        }
    })
}

#[bench]
fn bench_iter(b: &mut test::Bencher) {
    let mut stack = StrStack::new();
    for i in 0..100 {
        write!(stack, "{}", i);
    }

    b.iter(|| {
        for i in &stack {
            test::black_box(i);
        }
    })
}

#[bench]
fn bench_iter_naive(b: &mut test::Bencher) {
    let stack: Vec<_> = (0..100).map(|i| i.to_string()).collect();

    b.iter(|| {
        for i in &stack {
            test::black_box(i);
        }
    })
}

#[bench]
fn bench_alloc(b: &mut test::Bencher) {

    b.iter(|| {
        let mut stack = StrStack::new();
        for i in 0..100 {
            write!(stack, "{}", test::black_box(i));
        }
        test::black_box(stack);
    })
}

#[bench]
fn bench_alloc_naive(b: &mut test::Bencher) {

    b.iter(|| {
        let stack: Vec<_> = (0..100).map(|i| test::black_box(i).to_string()).collect();
        test::black_box(stack);
    })
}
