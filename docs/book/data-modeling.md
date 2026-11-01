# Order book data modeling

## Concept

An order book can be read from two sides, and each event may need both:

- **Live orders**, found by reference: the side, price and remaining shares of each order.
- **Prices**, found by side and price: the total shares waiting at that price.

Most ITCH events carry only a reference and, at most, a number of shares. The side and price must come from what the book stored when the order was added. The best bid is the highest buy price, the best ask is the lowest sell price, and the spread is the best ask minus the best bid.

### Exercise: nine events by hand

These events apply to one instrument. In each state below, the book lists sells on top and prices in descending order, so the best bid and best ask meet at the boundary between the two sides.

| # | Message | Event | Trade? |
|---|---|---|---|
| 1 | `A` | Reference 1: buy 100 at 10.00 | No |
| 2 | `A` | Reference 2: buy 50 at 10.00 | No |
| 3 | `A` | Reference 3: sell 200 at 10.05 | No |
| 4 | `A` | Reference 4: buy 70 at 9.98 | No |
| 5 | `E` | Reference 1: 30 shares executed | Yes, the aggressor sold |
| 6 | `X` | Reference 2: 20 shares cancelled | No |
| 7 | `U` | Reference 3 becomes 5: 150 shares at 10.04, same side | No |
| 8 | `D` | Reference 1: deleted | No |
| 9 | `E` | Reference 2: 30 shares executed | Yes, the aggressor sold |

#### Event 1

```text
Live orders:  ref 1: buy 100 at 10.00

Book:         buy  10.00: 100

Best bid: 10.00 | Best ask: none | Spread: none
```

#### Event 2

```text
Live orders:  ref 1: buy 100 at 10.00
              ref 2: buy 50 at 10.00

Book:         buy  10.00: 150 (2 orders)

Best bid: 10.00 | Best ask: none | Spread: none
```

#### Event 3

```text
Live orders:  ref 1: buy 100 at 10.00
              ref 2: buy 50 at 10.00
              ref 3: sell 200 at 10.05

Book:         sell 10.05: 200
              buy  10.00: 150 (2 orders)

Best bid: 10.00 | Best ask: 10.05 | Spread: 0.05
```

#### Event 4

A buy below the best bid gets its own price and does not change the best prices.

```text
Live orders:  ref 1: buy 100 at 10.00
              ref 2: buy 50 at 10.00
              ref 3: sell 200 at 10.05
              ref 4: buy 70 at 9.98

Book:         sell 10.05: 200
              buy  10.00: 150 (2 orders)
              buy   9.98: 70

Best bid: 10.00 | Best ask: 10.05 | Spread: 0.05
```

#### Event 5

The message has no side or price. They come from reference 1, a buy, so the aggressor sold.

```text
Live orders:  ref 1: buy 70 at 10.00
              ref 2: buy 50 at 10.00
              ref 3: sell 200 at 10.05
              ref 4: buy 70 at 9.98

Book:         sell 10.05: 200
              buy  10.00: 120 (2 orders)
              buy   9.98: 70

Best bid: 10.00 | Best ask: 10.05 | Spread: 0.05
```

#### Event 6

The shares decrease without a trade. Only the message type tells this apart from an execution.

```text
Live orders:  ref 1: buy 70 at 10.00
              ref 2: buy 30 at 10.00
              ref 3: sell 200 at 10.05
              ref 4: buy 70 at 9.98

Book:         sell 10.05: 200
              buy  10.00: 100 (2 orders)
              buy   9.98: 70

Best bid: 10.00 | Best ask: 10.05 | Spread: 0.05
```

#### Event 7

Reference 3 is removed and reference 5 is created. The side comes from reference 3, since `U` does not carry it. Price 10.05 had no other order, so it leaves the book, and the best ask improves.

```text
Live orders:  ref 1: buy 70 at 10.00
              ref 2: buy 30 at 10.00
              ref 4: buy 70 at 9.98
              ref 5: sell 150 at 10.04

Book:         sell 10.04: 150
              buy  10.00: 100 (2 orders)
              buy   9.98: 70

Best bid: 10.00 | Best ask: 10.04 | Spread: 0.04
```

#### Event 8

The message has only the reference. The 70 shares to remove come from the stored order. Reference 2 is still at 10.00, so the best bid does not change, even though an order left the best price.

```text
Live orders:  ref 2: buy 30 at 10.00
              ref 4: buy 70 at 9.98
              ref 5: sell 150 at 10.04

Book:         sell 10.04: 150
              buy  10.00: 30
              buy   9.98: 70

Best bid: 10.00 | Best ask: 10.04 | Spread: 0.04
```

#### Event 9

Reference 2 reaches zero shares and dies. Price 10.00 becomes empty, and it was the best bid, so the next best price must be found.

```text
Live orders:  ref 4: buy 70 at 9.98
              ref 5: sell 150 at 10.04

Book:         sell 10.04: 150
              buy   9.98: 70

Best bid: 9.98 | Best ask: 10.04 | Spread: 0.06
```

### What the exercise showed

**There are two paths to the data.** An event starts from a reference and reaches an order. A question about the best price starts from a side and reaches the prices.

**The best price changes in only a few cases:**

| Situation | Effect on the best price |
|---|---|
| An order enters | Compare it with the current best |
| An order leaves and its price still has shares | None |
| A price becomes empty, and it was not the best | None |
| The best price becomes empty | Find the next best price |

**Mistakes made by hand show what to check in code:**

- The total at a price must equal the sum of the live orders at that side and price.
- The two views can agree and still both be wrong. The message is the source of truth.
- A best bid at or above the best ask needs an explanation. The specification does not rule this out, so count these cases and find out when they happen.

## Where it appears in my code

Not in the code yet. It will first appear in step 3 of the [roadmap](../roadmap.md).

## Why it matters

This exercise is the basis of the book contract in the roadmap. How often each situation above happens determines which operations must be fast, and that can be measured on the real data. Across the full day of `12302019`, `A` and `D` alone make up about 86% of the messages.

The nine events also work as a first test case, since the expected state after each event is known.

## Related

- [Roadmap and book contract](../roadmap.md)
- [Fixed and growing memory](../memory/fixed-vs-growing-memory.md)

## Reference

- [Nasdaq TotalView-ITCH 5.0 specification](../spec/NQTVITCHspecification.pdf), sections 1.3 (Add Order) and 1.4 (Modify Order)
- [The Rust Book, 8: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
