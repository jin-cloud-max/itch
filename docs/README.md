# Documentation

## Project

- [Roadmap](roadmap.md): the steps of the project, what counts as done and the book contract.
- [Specification](spec/NQTVITCHspecification.pdf): the official Nasdaq TotalView-ITCH 5.0 specification.

## Concepts

### Order book

- [Data modeling](book/data-modeling.md): the two views of a book, worked through nine events by hand.

### Errors

- [Error propagation with `?`](errors/error-propagation.md): how `Result` and `?` return errors to the caller, including from `main`.

### I/O

- [Buffered reading](io/buffered-reading.md): how `BufReader`, `read` and `read_exact` behave when reading a file in streaming.
- [Length-prefixed framing](io/length-prefixed-framing.md): how a size prefix splits a stream of bytes into messages.

### Memory

- [Buffer reuse](memory/buffer-reuse.md): why a buffer created once avoids allocating for every message.
- [Direct-index counting](memory/direct-index-counting.md): how a small integer can be the position of its own counter.
- [Fixed and growing memory](memory/fixed-vs-growing-memory.md): why memory per kept item needs more care than a single large buffer.

### Numbers

- [Integer overflow](numbers/integer-overflow.md): why a counter too small for its value gives wrong numbers without an error in `--release`.
