# Write/Read JSON Sample Code

This code is intended to demo how `rust` + `serde` can be used to perform a typical application task
in a pleasant, readable, type-safe way. Namely, generate a big (50M message) JSON, and consume said
message.

I benchmarked it at 0m:30 write, 1m:00 read on my laptop. This compares with 6m:30 for simple python read.


## Quickstart

Create file:
```
cargo run --release -p build-json
```

Read file:
```
cargo run --release -p read-json
```

Python somewhat-equivalent: 
```
python3 -c 'import json; json.load(open("../log.json"))'
```