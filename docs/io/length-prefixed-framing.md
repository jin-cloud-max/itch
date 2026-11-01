# Length-prefixed framing

## Concept

Framing is how a stream of bytes is split into messages. With length-prefixed framing, each message is preceded by a number that gives its size, so the reader always knows how many bytes to read next.

The two common alternatives are:

- **Delimiters:** a special byte marks the end of a message, as newlines do in text files. Binary data can contain any byte value, so a delimiter needs escaping.
- **Fixed size:** every message has the same length. It needs no extra bytes, but it does not suit messages of different sizes.

In the ITCH files, the prefix is 2 bytes in big-endian. It counts the message and not the prefix itself, so `00 0c` is followed by 12 bytes.

## Where it appears in my code

`src/main.rs`, in `main`: the loop reads the 2-byte prefix with `read_exact` and converts it with `u16::from_be_bytes`.

## Why it matters

The prefix lets a reader find where each message ends without decoding it. A program can count or skip messages it does not care about by looking only at the prefix and the first byte.

The cost is that the stream has no recovery point. If the reader loses its position, every following prefix is read from the wrong place, and nothing in the data marks where the next message starts. Reading a message byte as a prefix produced wrong lengths in an early version of this program.

## Comparison with Go

The format is the same in any language. A Go version would use `io.ReadFull` and `binary.BigEndian.Uint16` where this one uses `read_exact` and `u16::from_be_bytes`.

## Related

- [Buffered reading](buffered-reading.md)

## Reference

- [`std::io::Read::read_exact`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact)
- [`u16::from_be_bytes`](https://doc.rust-lang.org/std/primitive.u16.html#method.from_be_bytes)
- [Rust Book, 4.3: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
