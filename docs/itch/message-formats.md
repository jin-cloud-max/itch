# ITCH message formats

## Concept

Each ITCH 5.0 message starts with the same four fields, followed by the fields of its type. This is a summary of the tables in the [specification](../spec/NQTVITCHspecification.pdf), which remains the source of truth. Each field is listed as name (offset, length in bytes). The total is the sum of the fields. The Spec column gives the section and page of each table.

**Common to every message:**

| Field | Offset | Length |
|---|---|---|
| Message type | 0 | 1 |
| Stock locate | 1 | 2 |
| Tracking number | 3 | 2 |
| Timestamp | 5 | 6 |

Integers are unsigned and big-endian. Text fields are ASCII, left-justified and padded with spaces. Prices are integers with implied decimals: 4 places, except in `V`, which has 8. Timestamps are nanoseconds since midnight. These rules are in [Data Types, page 4](../spec/NQTVITCHspecification.pdf#page=4).

The bytes do not say what they mean. The message type gives the layout, and the specification gives the kind of each field in its Value or Type column:

| Kind in the specification | Length | How to read it |
|---|---|---|
| Integer | 2, 4 or 8 | An unsigned integer of that exact size |
| Integer (timestamp) | 6 | No integer has 6 bytes, so it needs a larger type |
| Alpha | 1 | A single ASCII byte |
| Alpha | 2, 4 or 8 | An array of ASCII bytes |
| Price (4) | 4 | An unsigned integer with 4 implied decimal places |
| Price (8) | 8 | An unsigned integer with 8 implied decimal places |

### Orders

| Type | Message | Spec | Total | Fields after the common ones |
|---|---|---|---|---|
| `A` | Add Order | [1.3.1, p. 13](../spec/NQTVITCHspecification.pdf#page=13) | 36 | Reference (11, 8), side (19, 1), shares (20, 4), stock (24, 8), price (32, 4) |
| `F` | Add Order with MPID | [1.3.2, p. 13](../spec/NQTVITCHspecification.pdf#page=13) | 40 | Reference (11, 8), side (19, 1), shares (20, 4), stock (24, 8), price (32, 4), attribution (36, 4) |
| `E` | Order Executed | [1.4.1, p. 14](../spec/NQTVITCHspecification.pdf#page=14) | 31 | Reference (11, 8), executed shares (19, 4), match number (23, 8) |
| `C` | Order Executed with Price | [1.4.2, p. 14](../spec/NQTVITCHspecification.pdf#page=14) | 36 | Reference (11, 8), executed shares (19, 4), match number (23, 8), printable (31, 1), execution price (32, 4) |
| `X` | Order Cancel | [1.4.3, p. 15](../spec/NQTVITCHspecification.pdf#page=15) | 23 | Reference (11, 8), cancelled shares (19, 4) |
| `D` | Order Delete | [1.4.4, p. 15](../spec/NQTVITCHspecification.pdf#page=15) | 19 | Reference (11, 8) |
| `U` | Order Replace | [1.4.5, p. 15](../spec/NQTVITCHspecification.pdf#page=15) | 35 | Original reference (11, 8), new reference (19, 8), shares (27, 4), price (31, 4) |

### Trades

| Type | Message | Spec | Total | Fields after the common ones |
|---|---|---|---|---|
| `P` | Trade (Non-Cross) | [1.5.1, p. 16](../spec/NQTVITCHspecification.pdf#page=16) | 44 | Reference (11, 8), side (19, 1), shares (20, 4), stock (24, 8), price (32, 4), match number (36, 8) |
| `Q` | Cross Trade | [1.5.2, p. 17](../spec/NQTVITCHspecification.pdf#page=17) | 40 | Shares (11, 8), stock (19, 8), cross price (27, 4), match number (31, 8), cross type (39, 1) |
| `B` | Broken Trade | [1.5.3, p. 18](../spec/NQTVITCHspecification.pdf#page=18) | 19 | Match number (11, 8) |

### System and stock information

| Type | Message | Spec | Total | Fields after the common ones |
|---|---|---|---|---|
| `S` | System Event | [1.1, p. 4](../spec/NQTVITCHspecification.pdf#page=4) | 12 | Event code (11, 1) |
| `R` | Stock Directory | [1.2.1, p. 5](../spec/NQTVITCHspecification.pdf#page=5) | 39 | Stock (11, 8), market category (19, 1), financial status (20, 1), round lot size (21, 4), round lots only (25, 1), issue classification (26, 1), issue sub-type (27, 2), authenticity (29, 1), short sale threshold (30, 1), IPO flag (31, 1), LULD price tier (32, 1), ETP flag (33, 1), ETP leverage factor (34, 4), inverse indicator (38, 1) |
| `H` | Stock Trading Action | [1.2.2, p. 8](../spec/NQTVITCHspecification.pdf#page=8) | 25 | Stock (11, 8), trading state (19, 1), reserved (20, 1), reason (21, 4) |
| `Y` | Reg SHO Restriction | [1.2.3, p. 9](../spec/NQTVITCHspecification.pdf#page=9) | 20 | Stock (11, 8), Reg SHO action (19, 1) |
| `L` | Market Participant Position | [1.2.4, p. 9](../spec/NQTVITCHspecification.pdf#page=9) | 26 | MPID (11, 4), stock (15, 8), primary market maker (23, 1), market maker mode (24, 1), participant state (25, 1) |
| `V` | MWCB Decline Level | [1.2.5.1, p. 10](../spec/NQTVITCHspecification.pdf#page=10) | 35 | Level 1 (11, 8), level 2 (19, 8), level 3 (27, 8) |
| `W` | MWCB Status | [1.2.5.2, p. 10](../spec/NQTVITCHspecification.pdf#page=10) | 12 | Breached level (11, 1) |
| `K` | IPO Quoting Period Update | [1.2.6, p. 11](../spec/NQTVITCHspecification.pdf#page=11) | 28 | Stock (11, 8), release time (19, 4), release qualifier (23, 1), IPO price (24, 4) |
| `J` | LULD Auction Collar | [1.2.7, p. 11](../spec/NQTVITCHspecification.pdf#page=11) | 35 | Stock (11, 8), reference price (19, 4), upper price (23, 4), lower price (27, 4), extension (31, 4) |
| `h` | Operational Halt | [1.2.8, p. 12](../spec/NQTVITCHspecification.pdf#page=12) | 21 | Stock (11, 8), market code (19, 1), halt action (20, 1) |

### Auctions and other indicators

| Type | Message | Spec | Total | Fields after the common ones |
|---|---|---|---|---|
| `I` | Net Order Imbalance Indicator | [1.6, p. 18](../spec/NQTVITCHspecification.pdf#page=18) | 50 | Paired shares (11, 8), imbalance shares (19, 8), imbalance direction (27, 1), stock (28, 8), far price (36, 4), near price (40, 4), current reference price (44, 4), cross type (48, 1), price variation indicator (49, 1) |
| `N` | Retail Interest | [1.7, p. 19](../spec/NQTVITCHspecification.pdf#page=19) | 20 | Stock (11, 8), interest flag (19, 1) |
| `O` | Direct Listing with Capital Raise | [1.8, p. 20](../spec/NQTVITCHspecification.pdf#page=20) | 48 | Stock (11, 8), open eligibility (19, 1), minimum price (20, 4), maximum price (24, 4), near execution price (28, 4), near execution time (32, 8), lower collar (40, 4), upper collar (44, 4) |

## Where it appears in my code

`src/message.rs`, for the message types being decoded in step 2 of the [roadmap](../roadmap.md).

## Why it matters

The offsets say where to read each field, and the lengths say which integer type can hold it. The totals give the check for a length prefix.

Across the file of 2019-12-30, multiplying each count by its total and adding the 2-byte prefixes gives exactly the file size, 8,251,407,909 bytes. So the totals of the 18 types present agree with the real data. `B`, `N`, `O`, `W` and `h` do not appear in that file.

## Related

- [Length-prefixed framing](../io/length-prefixed-framing.md)
- [Data modeling](../book/data-modeling.md)

## Reference

- [Nasdaq TotalView-ITCH 5.0 specification, local copy](../spec/NQTVITCHspecification.pdf), sections 1.1 to 1.8
- [Nasdaq TotalView-ITCH 5.0 specification, latest version on the Nasdaq website](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHspecification.pdf)
- [The Rust Book, 6.1: Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
