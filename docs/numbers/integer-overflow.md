# Integer overflow

## Concept

Each integer type has a fixed range. A `u8` goes up to 255, and a `u64` goes up to about 18 quintillion.

When a value goes past the range, the result depends on the build:

- In debug builds, the program panics.
- In `--release` builds, the value wraps around to the start of the range, with no error. In a `u8`, 255 + 1 becomes 0.

An integer variable with no declared or inferred type is an `i32`, which goes up to about 2.1 billion.

## Where it appears in my code

`src/main.rs`, in `main`: `count` holds `u64` counters. An earlier version used `u8`.

## Why it matters

Wrapping produces wrong numbers that look valid. With `u8` counters, the sample reported 112 messages of type `A`, when the real count is 1,360,240. The remainder of 1,360,240 divided by 256 is exactly 112. The program ran without errors, and types with small counts, such as `S`, were correct, which made the problem harder to notice.

Since performance is measured in `--release`, this project runs in the mode that gives no warning. The type must fit the largest value the code can reach, including sums such as the total shares at a price.

Methods such as `checked_add`, `wrapping_add` and `saturating_add` make the intended behavior explicit.

## Comparison with Go

Go also wraps around on overflow, but in every build, and never panics. Rust panics in debug builds, which helps to find the problem during development.

## Related

- [Direct-index counting](../memory/direct-index-counting.md)

## Reference

- [The Rust Book, 3.2: Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
