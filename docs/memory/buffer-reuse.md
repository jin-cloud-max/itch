# Buffer reuse

## Concept

A buffer created once outside a loop can serve every iteration. Each read overwrites the part of the buffer it needs, so the loop itself does not allocate.

The alternative is a new `Vec` for each item. Each one costs an allocation and a release, and `vec![0u8; n]` also fills the memory with zeros before it is read into.

| Option | Allocations in the loop | Work per message |
|---|---|---|
| Fixed array | None | The read |
| Reused `Vec` | A few, only when it must grow | The read, plus `resize` |
| New `Vec` per message | One per message | Allocation, zeroing, the read and the release |

An array of a fixed size lives on the stack here, while a `Vec` keeps its data on the heap.

Only the first `n` bytes of a reused buffer are valid for the current message. The rest can hold leftovers from earlier messages.

## Where it appears in my code

`src/main.rs`, in `main`: `prefix` and `message` are arrays created before the loop and overwritten on each iteration.

## Why it matters

`message` has 65,535 bytes because that is the largest value the 2-byte prefix can give, so no message can overflow it. Those 64 KB are reserved and zeroed once, and each iteration touches only the first bytes.

A cost per message is multiplied by the number of messages. `data/sample.bin` alone has about 3.5 million, so a new `Vec` for each message would mean about 3.5 million allocations.

A larger buffer costs memory, but not necessarily time. This has not been measured yet in this project, and it is not assumed to be the program's bottleneck.

## Comparison with Go

Reusing a `[]byte` declared outside the loop has the same effect. Allocating a slice with `make` in each iteration also adds work for the garbage collector. Rust has no garbage collector, so the cost is the allocator's, and each `Vec` is released when it goes out of scope.

## Related

- [Fixed and growing memory](fixed-vs-growing-memory.md)
- [Buffered reading](../io/buffered-reading.md)
- [Length-prefixed framing](../io/length-prefixed-framing.md)

## Reference

- [The Rust Book, 4.1: The Stack and the Heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
- [`Vec::resize`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize)
