# Contents
This crate contains some basic useful macros to use in my projects. It has:
- [`timeprintln!`](#timeprintln)
- [`perf_time!`](#perf_time)
- [`Add`, `Sub`, `Div`, `Mul`, and `PartialOps`](#operator-proc-macros)
---
# Adding this crate
Into your `Cargo.toml` under `[dependencies]` add:
```toml
utils = { git = "https://github.com/s-0fia/custom-util-rs" }
```
---
# `timeprintln!`
Prints to the standard output with a new line, and with the time before it.

This macro uses the same syntax as the [`println`](https://doc.rust-lang.org/std/macro.println.html) macro, but with extra optional formatting options.

By default the format is `[%H:%M:%S]` in your local system time.

For custom formatting of time, this macro uses [`chrono::Format`](https://docs.rs/chrono/latest/chrono/format/index.html) which closely resembles the C `strftime` format. A full list of formatting options can be found [here](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

## Examples
### Default Format Usage
Using the default time format works the same as [`println`]:
```rust
use utils::timeprintln;

timeprintln!();
let foo = 20;
timeprintln!("Foo is {foo}.");
let bar = vec![15];
timeprintln!("Bar[0] is {}.", bar[0]);
```

### Custom Format Usage
If you wish to use your own format such as `[%Y/%m/%d-%H:%M:%S]`, you can do:
```rust
use utils::timeprintln;

timeprintln!(as "[%Y/%m/%d-%H:%M:%S]");
let foo = 20;
timeprintln!(as "[%Y/%m/%d-%H:%M:%S]"; "Foo is {foo}.");
```
---
# `perf_time!`

Times a performance of a function over many iteractions and returns the
elapsed time in a [`std::time::Duration`](https://doc.rust-lang.org/stable/core/time/struct.Duration.html) struct.

*Note: As the data is discarded when ran with `--release` it will
return 0ns elapsed time as the code is optimised out.*

This macro has three sets of input:
1. Function name only
2. Number of iterations and function name
3. Data to input and closure to run the function with

For case 1. it defaults to 10⁶ iterations.

For case 2. it runs `1..=iterations given` times with no data passed to
the function.

For case 3. it iterates over the data and uses the closure provided.

## Examples
```rust
use utils::perf_time;

fn foo() {
    // Some workload.
}

fn bar(n: isize) {
    // Some workload.
}

fn bazz(a: isize, b: f64) {
    // Some workload.
}

// Case 1.
let foo_time = perf_time!(foo);
// Case 2.
let foo_time = perf_time!(5_000_000, foo);

// Case 3.
let data: Vec<_> = (0..5_000_000).collect();
let bar_time = perf_time!(data, |n| bar(n));
let data: Vec<_> = (0..5_000_000).map(|n| (n, n as f64)).collect();
let bazz_time = perf_time!(data, |(a, b)| bazz(a, b));
```
---
# Operator Proc Macros
Derive macro for the impls of the traits [`std::ops::Add`](https://doc.rust-lang.org/stable/core/ops/trait.Add.html), [`std::ops::Sub`](https://doc.rust-lang.org/stable/core/ops/trait.Sub.html), [`std::ops::Mul`](https://doc.rust-lang.org/stable/core/ops/trait.Mul.html), and [`std::ops::Div`](https://doc.rust-lang.org/stable/core/ops/trait.Div.html) for types that implement [`Into<T>`](https://doc.rust-lang.org/stable/core/convert/trait.Into.html) where T = the struct. This is a naïve implementation which applys the operation each field together of the LHS and RHS.

`PartialOps` will simply derive all four of the operations in the same way.

*Note: This proc macro is restricted to only named and unnamed structs.*

## Example
```rust
use utils::Add;

#[derive(Add)]
struct Foo {
    x: f64,
    y: f64,
}
```
Will expand to:
```rust
struct Foo {
    x: f64,
    y: f64,
}

impl<T : Into<Foo>> std::ops::Add<T> for Foo {
    type Output = Self;
    
    fn add(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
```