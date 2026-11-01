# Roadmap

The goal is a correct order book, rebuilt from one trading day of ITCH 5.0 data, measured at every step. Later analyses build on it.

Do the steps in order. Mark a task as done only when its result has been checked.

## Step 1: Read the file and count messages

Each message in the file is preceded by a 2-byte big-endian length. This framing is not in the specification; it was found by inspecting `data/sample.bin` with `xxd`.

- [x] Take the file path as a command-line argument.
- [x] Read the file in streaming, without loading it all into memory.
- [x] Count messages per type and print the counts and the total.
- [x] Handle the three ways a file can end: before a length prefix (normal), inside a prefix, or inside a message. Report a cut message instead of crashing.
- [x] Check the result: the sizes of all messages read, plus their prefixes, must add up to the file size, minus any cut message.
- [x] Run on `data/sample.bin` in `--release`.
- [x] Run on the full day in `--release`. The file was decompressed to disk (8,251,407,909 bytes), so performance runs do not depend on `gzip`.

Questions this step answers:

- [x] Which message types appear in this 2019 file? The specification is from 2024, so check `N` and `O` in particular. **Answer:** `A`, `C`, `D`, `E`, `F`, `H`, `I`, `J`, `K`, `L`, `P`, `Q`, `R`, `S`, `U`, `V`, `X` and `Y`. `N` and `O` do not appear, and neither do `W` and `h`.
- [x] What fraction of the add orders are `F` rather than `A`? **Answer:** about 1.25% (1,485,888 of 118,631,456). This is a share of orders, not of traded volume.
- [x] Which message types are the most frequent? **Answer:** of 268,744,780 messages, `A` is 43.6%, `D` is 42.6% and `U` is 8.1%.

## Step 2: Decode the messages

- [ ] Decode every message type according to the specification.
- [ ] Choose what happens with an unknown type.
- [ ] Verify that each length prefix matches the size the specification gives for that type.
- [ ] Decode prices, timestamps and text fields correctly.

Questions this step answers:

- [ ] How many instruments exist on this day?
- [ ] What fraction of the traded volume comes from `P`?
- [ ] Do `E` messages appear for the crosses, or only non-printable `C` messages?

## Step 3: Rebuild the book for one instrument

The book covers displayed Nasdaq orders only. It does not show hidden liquidity, other exchanges or the identity of the aggressor.

- [ ] Use `R` to find the stock locate of the chosen instrument.
- [ ] Apply the events in the contract below.
- [ ] Implement the queries in the contract below.
- [ ] Count every failed check, without stopping the program.
- [ ] Reproduce by test the nine-event exercise in [data modeling](book/data-modeling.md).
- [ ] Run the whole day for this instrument and inspect the check counters.

Start with the simplest correct version. Add counters to learn how the data behaves before choosing the final structure:

- [ ] Number of distinct prices per side, typical and maximum.
- [ ] Distance from the best price for each operation.
- [ ] How often a price is created or emptied.
- [ ] How often the best price is emptied.
- [ ] Largest number of live orders at the same time.
- [ ] Number of events with an unknown reference. Do all orders start in this file?

### Book contract

**State, per instrument:**

- Live orders, found by reference: side, price and remaining shares.
- Prices, found by side and price: total shares.

**Events:**

| Message | Effect |
|---|---|
| `A`, `F` | Create the order and add its shares to the price |
| `E`, `C`, `X` | Reduce the order and the price, using the price stored in the order |
| `D` | Remove the order's remaining shares from the order and the price |
| `U` | Do a `D` on the old reference and create the new one, keeping the side |

An order with zero shares leaves the live orders. A price with zero shares leaves the prices. `C` carries the execution price, which is not the price where the order sits in the book.

**Queries:**

- Best bid, best ask and spread.
- Total shares at a side and price.
- Side, price and shares of a reference.
- Ordered list of prices on each side (depth).

**Checks, counted without stopping the program:**

- Unknown reference.
- `A` or `F` with a reference that already exists.
- A reduction larger than the order's remaining shares.
- A price total different from the sum of its orders. This check is expensive, so run it only occasionally.
- A crossed book (best bid at or above best ask), split by trading state. The specification does not rule this out, so find out when it happens.

**Left out of the first version:** the sequence of orders within a price, MPID, dead orders and the number of orders per price.

## Step 4: Generalize to all instruments

- [ ] Run the book for every instrument on the full day.
- [ ] Set a memory limit with `ulimit -v` before running.
- [ ] Measure peak memory and the largest number of live orders across the market.
- [ ] Inspect the check counters across all instruments.

## Step 5: Optimize, one change at a time

- [ ] Record the baseline in `docs/experiments.md`: total time with `hyperfine`, p50, p99 and p99.9 latency per message, a `samply` profile and `dhat-rs` allocations.
- [ ] Before each change, write down what will be measured and on which data.
- [ ] Make one change, measure it, and record the before, the after and the reason.

To trust a result:

- [ ] Run in `--release`, on data that includes market hours. `data/sample.bin` is only the start of the day.
- [ ] Confirm that both versions produce the same final book.
- [ ] Keep the behavior counters out of timed runs.
- [ ] Treat a difference smaller than the variation between runs as no result.

## Later ideas

Each one builds on the book, and none is committed to yet:

- How many times faster than real time the program runs, on average and in the busiest second.
- Order lifetime, and how many orders are executed or cancelled.
- Queue position of a hypothetical order.
- Live replay, using threads.
- Behavior of identified participants by MPID. It always covers only `F` orders resting on the book, so show its share of the traded volume alongside it.

Left out: profit and loss per participant, since the feed does not identify the aggressor, and a matching engine, which would be a separate project.
