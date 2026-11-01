# Fixed and growing memory

## Concept

Before worrying about the size of something, ask how many of it will exist at the same time.

- **Fixed memory:** there is one of it, however large the input is. A message is read, used and discarded, so its buffer does not grow.
- **Growing memory:** there is one per item that must be kept. Each extra byte is multiplied by the number of items.

This example uses made-up numbers to show the difference:

| Item | With 1 thousand | With 10 million |
|---|---|---|
| One 64 KB buffer | 64 KB | 64 KB |
| One order of 50 bytes each | 50 KB | 500 MB |
| One order of 100 bytes each | 100 KB | 1 GB |

For the order book, the number that counts is the peak of live orders, not the size of the file. Orders leave the book when they die, so the total rises and falls during the day.

## Where it appears in my code

`src/main.rs`, in `main`: the `message` buffer is fixed memory. Growing memory does not exist in the code yet. It will first appear with the live orders of the book, in step 3 of the [roadmap](../roadmap.md).

## Why it matters

Spending memory on the fixed buffer is a good trade, since 64 KB is negligible and the cost never grows. The same reasoning does not carry over to data kept per order.

This does not mean the design has to change in advance. The plan is to build the simplest correct book, measure the peak of live orders and the memory used, and only then decide whether anything must change.

## Comparison with Go

The reasoning is the same. In Go, a growing number of live objects also adds work for the garbage collector, which has to go through them. Rust has no such cost, but the memory used by the data still grows.

## Related

- [Buffer reuse](buffer-reuse.md)

## Reference

- [The Rust Book, 4.1: The Stack and the Heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
- [The Rust Book, 8: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
