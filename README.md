# Simple-Discrete-Time-System #

Simple discrete time system library for rust. This library aims to simplify the creation of a discrete time system or filter.

## Simple Example ##
Lets say we want to implement the following filter:

![equation](https://latex.codecogs.com/png.image?\inline&space;\dpi{110}\bg{white}y[n]=0.9\times&space;x[n]&plus;0.1\times&space;y[n-1])

We can implement the filter using the difference equation in rust as follows:

```rust
use sdts::{create_system, SystemCreator};

let filter1 = create_system!(1,2,|x,y| {
    y[0] = 0.9*x[0] + 0.1*y[-1];
});
```

What is created is a system that can be used to process data:

```rust
let x: [f64; N] = [ /* Some Data to populate */];
let mut y: [f64; N];


for i in 0..N {
    y[i] = filter1.process(x[i]);
}
```