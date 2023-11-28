pub use util_derive::{Add, Sub, Mul, Div, PartialOps};
pub use chrono;

/// Times a performance of a function over many iteractions and returns the
/// elapsed time in a [`std::time::Duration`] struct.
/// 
/// *Note: As the data is discarded when ran with `--release` it will
/// return 0ns elapsed time as the code is optimised out.*
/// 
/// This macro has three sets of input:
/// 1. Function name only
/// 2. Number of iterations and function name
/// 3. Data to input and closure to run the function with
/// 
/// For case 1. it defaults to 10⁶ iterations.
/// 
/// For case 2. it runs `1..=iterations passed` times with no data passed to
/// the function.
/// 
/// For case 3. it iterates over the data and uses the closure provided.
/// 
/// # Examples
/// ```
/// use utils::perf_time;
/// 
/// fn foo() {
///     // Some workload.
/// }
///
/// fn bar(n: isize) {
///     // Some workload.
/// }
/// 
/// fn bazz(a: isize, b: f64) {
///     // Some workload.
/// }
/// 
/// // Case 1.
/// let foo_time = perf_time!(foo);
/// // Case 2.
/// let foo_time = perf_time!(5_000_000, foo);
/// 
/// // Case 3.
/// let data: Vec<_> = (0..5_000_000).collect();
/// let bar_time = perf_time!(data, |n| bar(n));
/// let data: Vec<_> = (0..5_000_000).map(|n| (n, n as f64)).collect();
/// let bazz_time = perf_time!(data, |(a, b)| bazz(a, b));
/// ```
#[macro_export]
macro_rules! perf_time {
    ($f: ident) => { perf_time!(1_000_000, $f) };
    ($iters: expr, $f: ident) => {
        {
            let r = 1..=$iters;

            let start = std::time::Instant::now();

            for _ in r {
                let _ = $f();
            }

            start.elapsed()
        }
    };
    ($data: expr, $f: expr) => {
        {
            let data = $data;

            let start = std::time::Instant::now();
            for d in data {
                let _ = $f(d);
            }

            start.elapsed()
        }
    };
}

/// Prints to the standard output with a new line, and with the time
/// before it.
/// 
/// This macro uses the same syntax as the [`println`] macro, but with
/// extra optional formatting options.
///
/// By default the format is `[%H:%M:%S]` in your local system time.
/// 
/// For custom formatting of time, this macro uses [`chrono::Format`]
/// which closely resembles the C `strftime` format. A full list of
/// formatting options can be found
/// [here](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).
/// 
/// # Examples
/// ## Default Format Usage
/// Using the default time format works the same as [`println`]:
/// ```
/// use utils::timeprintln;
/// 
/// timeprintln!();
/// let foo = 20;
/// timeprintln!("Foo is {foo}.");
/// let bar = vec![15];
/// timeprintln!("Bar[0] is {}.", bar[0]);
/// ```
/// 
/// ## Custom Format Usage
/// If you wish to use your own format such as `[%Y/%m/%d-%H:%M:%S]`,
/// you can do:
/// ```
/// use utils::timeprintln;
/// 
/// timeprintln!(as "[%Y/%m/%d-%H:%M:%S]");
/// let foo = 20;
/// timeprintln!(as "[%Y/%m/%d-%H:%M:%S]"; "Foo is {foo}.");
/// ```
#[macro_export]
macro_rules! timeprintln {
    () => { 
        timeprintln!(as "[%H:%M:%S]")
    };
    (as $f:expr) => {
        println!("{}", utils::chrono::Local::now().format($f))
    };
    (as $f:expr; $($arg:tt)*) => {
        {
            print!("{} ", utils::chrono::Local::now().format($f));
            println!($( $arg )*)
        }
    };
    ($($arg:tt)*) => {
        timeprintln!(as "[%H:%M:%S]"; $( $arg )*)
    };
}