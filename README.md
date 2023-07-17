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
python3 run.py
```


## Demo outline

* Intention - show how Rust can be used to undertake typical task, live code
* The task:
  - parse large (/HUGE) log file in JSON format
  - fetch latencies for http requests only
* "Heres one I made earlier" - generate and `cat` the format
* `cargo new`
* 'Dumb version': use serde, load whole thing as value
* Pretty straightforward translation of python
* Very easy but quite clunky and not any faster than python
  - Not a big surprise, python json parser is very fast C code
* Now lets use the full power of Rust - we will end up with code
  that is faster and more correct
* The key difference between python/v1 and v2 is 'what we know about our data'
* In python/v1 we know very little, key error will cause runtime crash
* In v2, we know exactly what our data looks like, strongly typed
* After the parsing stage, impossible for the rest of the code to fail
* "I did try on larger files... the were q a bit faster in Rust'
