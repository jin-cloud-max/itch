# Direct-index counting

## Concept

When the counted value is a small integer, it can be the position of its own counter in an array. There is nothing to search for, since the value already says which counter to change.

A `u8` has 256 possible values, so an array of 256 counters covers all of them. An array index must be a `usize`, and `usize::from` converts a `u8` without any possible loss.

Iterating with `iter().enumerate()` gives each position with its value, so the position can be turned back into the original byte for printing.

## Where it appears in my code

`src/main.rs`, in `main`: `count` has one `u64` counter for each value of the message type byte, `message[0]`.

## Why it matters

The array has a fixed size, whatever the size of the input, and the counters are not mixed with the data being read. The message buffer is overwritten on every read, so it cannot hold results.

Using the message size as the index would count how many messages have each size, not each type. Different types can have the same size, so the two counts are not interchangeable.

## Comparison with Go

The Go version is the same: `var count [256]uint64` and `count[b]++`. A `byte` can index an array directly in Go, while Rust requires a `usize`.

## Related

- [Integer overflow](../numbers/integer-overflow.md)
- [Fixed and growing memory](fixed-vs-growing-memory.md)

## Reference

- [The Rust Book, 3.2: The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
- [`Iterator::enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)
