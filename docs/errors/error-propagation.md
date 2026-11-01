# Error propagation with `?`

## Concept

Rust represents recoverable errors with `Result<T, E>`. The `?` operator unwraps an `Ok` value. On an `Err`, it returns the error from the current function right away.

Since `?` can return early, the function must have a return type that can hold that error. `main` can return `Result`, and when it returns `Err`, the program prints the error and exits with a failure code.

A function's final expression is its return value only if it has no trailing semicolon. `Ok(())` returns a value, while `Ok(());` discards it.

## Where it appears in my code

`src/main.rs`, in `main`: it returns `std::io::Result<()>` and uses `?` on `File::open` and on the read call.

## Why it matters

Errors stay visible in the function's signature, and the compiler rejects code that forgets to handle them. A missing input file becomes an error message, not a crash in the middle of the program.

`panic!` is meant for bugs and states the program cannot recover from. Expected failures, such as a bad path, belong in a `Result`.

## Comparison with Go

`?` plays the role of `if err != nil { return err }`, in a single character. In Go, ignoring a returned error is possible, for example by assigning it to `_`, or by not assigning the result at all. In Rust, an unused `Result` produces a compiler warning, and getting at the value inside requires dealing with the error case.

## Related

- [Buffered reading](../io/buffered-reading.md)

## Reference

- [The Rust Book, 9.2: Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
