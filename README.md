# itch

A Nasdaq TotalView-ITCH 5.0 binary feed parser and order book reconstruction in Rust. This is a learning project focused on performance, memory and latency.

## Use of AI

I use [Claude Code](https://claude.com/claude-code) as a study assistant on this project, not to write the implementation. I write all the code myself. Claude explains concepts, reviews my code and points me in the right direction, usually with questions and hints rather than finished solutions.

[CLAUDE.md](CLAUDE.md) sets these rules, which is why it tells Claude not to change the code. Claude can edit the concept notes in `docs/`, and it also helped write this README.

## Data

Trading day files go in `data/`, which is not tracked by git. Each compressed file is between 3.5 and 5.6 GB, and much larger once decompressed.

To download the smallest available file (December 30, 2019, 3.52 GB), run this from the repository root:

```bash
mkdir -p data && cd data
BASE='https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH'
curl -fL -C - -O "$BASE/12302019.NASDAQ_ITCH50.gz"
gzip -t 12302019.NASDAQ_ITCH50.gz && echo "file OK"
```

`-C -` resumes the download if the connection drops. `gzip -t` checks the file's integrity without writing anything to disk, and takes a few minutes. The `.md5sum` files appear in the listing, but downloading them returns a 404. Other trading days are available at <https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/>.

To create a 100 MB sample without decompressing the whole file:

```bash
gzip -dc 12302019.NASDAQ_ITCH50.gz | head -c 100M > sample.bin
```

The file is cut by byte count, so the last message may be incomplete.

## Specification

The specification is in [docs/spec/NQTVITCHspecification.pdf](docs/spec/NQTVITCHspecification.pdf). The latest version is on the [Nasdaq website](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHspecification.pdf).
