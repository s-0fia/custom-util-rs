#[cfg(feature = "derive")]
pub use util_derive::{Add, Sub, Mul, Div, PartialOps};
#[cfg(feature = "timeprint")]
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
#[cfg(feature = "perf")]
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

/// Prints to the standard output, with the time before it.
/// 
/// This macro uses the same syntax as the [`print!`] macro, but with
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
/// Using the default time format works the same as [`print!`]:
/// ```
/// use utils::timeprint;
/// 
/// timeprint!();
/// let foo = 20;
/// timeprint!("Foo is {foo}.");
/// let bar = vec![15];
/// timeprint!("Bar[0] is {}.", bar[0]);
/// ```
/// 
/// ## Custom Format Usage
/// If you wish to use your own format such as `[%Y/%m/%d-%H:%M:%S]`,
/// you can do:
/// ```
/// use utils::timeprint;
/// 
/// timeprint!(as "[%Y/%m/%d-%H:%M:%S]");
/// let foo = 20;
/// timeprint!(as "[%Y/%m/%d-%H:%M:%S]"; "Foo is {foo}.");
/// ```
#[macro_export]
#[cfg(feature = "timeprint")]
macro_rules! timeprint {
    () => { 
        timeprint!(as "[%H:%M:%S]")
    };
    (as $f:expr) => {
        print!("{}", utils::chrono::Local::now().format($f))
    };
    (as $f:expr; $($arg:tt)*) => {
        {
            print!("{} ", utils::chrono::Local::now().format($f));
            print!($( $arg )*)
        }
    };
    ($($arg:tt)*) => {
        timeprint!(as "[%H:%M:%S]"; $( $arg )*)
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
#[cfg(feature = "timeprint")]
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

/// Lots of ansi color escape code constants and a function to
/// replace using the architecture in strings.
/// 
/// *Note: You can use the codes directly in a [`format!`] string.
/// See plain usage examples below*
/// 
/// # Architecture
/// ---
/// ## Style codes
/// | Name | String | Meaning |
/// | ---- | ------ | ------- |
/// | [`X`] | `"_X"` | Clears all ANSI formatting |
/// | [`style::BLD`] | `"_BLD"` | Bold |
/// | [`style::ITL`] | `"_ITL"` | Italicized |
/// | [`style::UND`] | `"_UND"` | Underline |
/// ---
/// ## Color codes
/// | Foreground | String | Background | String | Meaning |
/// | ---------- | ------ | ---------- | ------ | ------- |
/// | [`f_color::BLK`] | ".BLK" | [`b_color::BLK`] | "#BLK" | Black |
/// | [`f_color::RED`] | ".RED" | [`b_color::RED`] | "#RED" | Red |
/// | [`f_color::GRN`] | ".GRN" | [`b_color::GRN`] | "#GRN" | Green |
/// | [`f_color::YLW`] | ".YLW" | [`b_color::YLW`] | "#YLW" | Yellow |
/// | [`f_color::BLU`] | ".BLU" | [`b_color::BLU`] | "#BLU" | Blue |
/// | [`f_color::MGT`] | ".MGT" | [`b_color::MGT`] | "#MGT" | Magenta |
/// | [`f_color::CYN`] | ".CYN" | [`b_color::CYN`] | "#CYN" | Cyan |
/// | [`f_color::WHT`] | ".WHT" | [`b_color::WHT`] | "#WHT" | White |
/// ---
/// # Examples
/// ## Using consts directly
/// ```
/// use utils::ansi::{style::*, f_color::*, *};
/// 
/// let manual = String::from("\x1b[1m\x1b[3m\x1b[32mHello, world!\x1b[0m");
/// let output = format!("{BLD}{ITL}{GRN}Hello, world!{X}");
/// assert_eq!(output, manual);
/// ```
#[cfg(feature = "ansi")]
pub mod ansi;

#[macro_export]
macro_rules! flush {
    () => {
        std::io::stdout().flush().unwrap();
    };
}