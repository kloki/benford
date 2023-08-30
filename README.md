# Benford's law

https://en.wikipedia.org/wiki/Benford's_law

```
cargo run --release -- ./data/random.txt
cargo run --release -- ./data/poweroftwo.txt
```

## Output example

```
Leading digit distribution

      |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9
TARGET|0.301|0.176|0.125|0.097|0.079|0.067|0.058|0.051|0.046
FOUND |0.300|0.180|0.120|0.100|0.080|0.065|0.055|0.055|0.045

Significance level: 0.05
Found distance score: 0.1257515453736097

✅ PASS!! data set seems to be natural!

warning: Make sure the category of data your testing follows Benford's law.
https://en.wikipedia.org/wiki/Benford's_law
```
