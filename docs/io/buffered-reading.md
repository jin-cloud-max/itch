# Buffered reading

## Concept

Each read from a `File` can become a system call, which is expensive compared with ordinary code. `BufReader` wraps a reader and fetches larger blocks from it, then serves later small reads from memory.

`read` fills as much of the given buffer as is available and returns how many bytes it read. That number can be smaller than the buffer, and 0 means the end of the file. `read_exact` fills the whole buffer, or returns an error if the data runs out first.

## Where it appears in my code

`src/main.rs`, in `main`: the `File` is wrapped in a `BufReader`, and the loop reads from it with `read`.

## Why it matters

Streaming keeps memory use independent of the file size, which matters for a trading day larger than the available RAM.

The boundaries of a `read` come from the buffer and the data available, not from the content of the file. A format made of messages cannot rely on them, because a message can be split between two reads.

## Comparison with Go

`BufReader` is close to `bufio.Reader`. `Read::read` behaves like `io.Reader.Read`, which can also return fewer bytes than requested, and `read_exact` is close to `io.ReadFull`.

## Related

- [Error propagation with `?`](../errors/error-propagation.md)

## Reference

- [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
- [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html)
- [The Rust Book, 12.2: Reading a File](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
