# robogen
Like cuddly robots, but making them too

## Running

Run with
```
$ cargo run > timing.dat
```

This will generate a 32768 byte file containing the timing data for a VGA signal.
Over stderr an ascii reprsentation is printed out as well.

## Analyzing

To analyze the waveforms in a single frame:

```
< <( <robogen/timing.dat head -c "$((50*525+1))" ) cargo run --manifest-path=robogen/Cargo.toml --bin analyze
```

Or to analyze multiple frames (here, 3) in succession:

```
< <( for i in {0..2} ; do <robogen/timing.dat head -c "$((50*525+1))" ; done ) cargo run --manifest-path=robogen/Cargo.toml --bin analyze
```

## Uploading

Upload the `timing.dat` file onto the chip using the minipro programmer using the command:

```
minipro -p 'AT28C256' -w timing.dat
```
