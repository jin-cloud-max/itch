# Fixed-width fields

## Concept

A fixed-width field reserves enough space for the largest value it can hold, and any shorter value fills the rest with padding. In ITCH, text fields are left-justified and padded with spaces on the right, so the symbol `A` takes the same 8 bytes as `ARGX`:

```text
Offset:  24 25 26 27 28 29 30 31
ARGX:     A  R  G  X  _  _  _  _
A:        A  _  _  _  _  _  _  _
```

The trade-off:

| | Fixed width | Variable width |
|---|---|---|
| Memory | Wastes bytes on short values | Uses only what is needed |
| Finding a field | Go directly to its position | Scan or calculate the position |
| Validation | Simple | More cases to handle |
| Limit | A larger value does not fit without changing the format | More flexible |

ITCH uses both ideas. Messages have variable sizes, marked by the length prefix, and each message type has fixed positions inside.

To show a padded field as text, `std::str::from_utf8` and `trim_end` give a view of the same bytes, without copying or allocating. A `String` would own a copy of the text on the heap.

## Where it appears in my code

- `src/message.rs`, in `MessageBody`: `stock` in `AddOrder` is a `[u8; 8]`, the same shape the specification uses.
- `src/main.rs`, in `main`: the `message` buffer applies the same reasoning of reserving for the largest case. It holds 65,535 bytes, the largest value of the prefix, but it is a reusable buffer and not a field of the format.

## Why it matters

With fixed positions, the price of an `A` message is always at offset 32, whatever the symbol. The reader can go straight to it, with no separators and no size for each field.

A `String` per symbol would mean one allocation for each of the roughly 117 million `A` messages in the file of 2019-12-30, and the book identifies instruments by stock locate, so most of those strings would never be used. This allocation cost has not been measured yet.

The width must come from a real limit. Appendix B of the specification says Nasdaq restricts symbols to a maximum of 8 characters, so 8 bytes are enough. Without a known limit, a fixed width can truncate data or reject valid values.

The same reasoning applies elsewhere: a format or struct with a known limit can trade some bytes for direct access. The saving depends on how many values exist, so this also depends on [fixed and growing memory](fixed-vs-growing-memory.md).

## Comparison with Go

A `[8]byte` works the same way and also lives inline in its struct. A Go `string` is a pointer and a length, 16 bytes on a 64-bit system, with its text stored elsewhere. It is immutable, while a Rust `String` is growable and takes 24 bytes before its text.

## Related

- [Length-prefixed framing](../io/length-prefixed-framing.md)
- [ITCH message formats](../itch/message-formats.md)
- [Buffer reuse](buffer-reuse.md)
- [Fixed and growing memory](fixed-vs-growing-memory.md)

## Reference

- [Nasdaq TotalView-ITCH 5.0 specification](../spec/NQTVITCHspecification.pdf), [Data Types, page 4](../spec/NQTVITCHspecification.pdf#page=4) and [Appendix B, page 31](../spec/NQTVITCHspecification.pdf#page=31)
- [The Rust Book, 4.3: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [The Rust Book, 8.2: Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
