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
pub mod ansi {
    pub const REPLACE_MAP: [(&str, &str); 20] = [
        ("_X", self::X), ("_BLD", style::BLD), ("_ITL", style::ITL), ("_UND", style::UND),
        (".BLK", f_color::BLK), (".RED", f_color::RED), (".GRN", f_color::GRN), (".YLW", f_color::YLW), (".BLU", f_color::BLU), (".MGT", f_color::MGT), (".CYN", f_color::CYN), (".WHT", f_color::WHT),
        ("#BLK", b_color::BLK), ("#RED", b_color::RED), ("#GRN", b_color::GRN), ("#YLW", b_color::YLW), ("#BLU", b_color::BLU), ("#MGT", b_color::MGT), ("#CYN", b_color::CYN), ("#WHT", b_color::WHT)
    ];

    // Denoted by a _
    pub const X:       &str = "\x1b[0m";  // Clear code

    pub mod f_color { // Denoted by a .
        pub const BLK: &str = "\x1b[30m"; // Black
        pub const RED: &str = "\x1b[31m"; // Red
        pub const GRN: &str = "\x1b[32m"; // Green
        pub const YLW: &str = "\x1b[33m"; // Yellow
        pub const BLU: &str = "\x1b[34m"; // Blue
        pub const MGT: &str = "\x1b[35m"; // Magenta
        pub const CYN: &str = "\x1b[36m"; // Cyan
        pub const WHT: &str = "\x1b[37m"; // White
    }

    pub mod b_color { // Denoted by a #
        pub const BLK: &str = "\x1b[40m"; // Black
        pub const RED: &str = "\x1b[41m"; // Red
        pub const GRN: &str = "\x1b[42m"; // Green
        pub const YLW: &str = "\x1b[43m"; // Yellow
        pub const BLU: &str = "\x1b[44m"; // Blue
        pub const MGT: &str = "\x1b[45m"; // Magenta
        pub const CYN: &str = "\x1b[46m"; // Cyan
        pub const WHT: &str = "\x1b[47m"; // White
    }
    
    pub mod style { // Denoted by a _
        pub const BLD: &str = "\x1b[1m";  // Bold
        pub const ITL: &str = "\x1b[3m";  // Italicized
        pub const UND: &str = "\x1b[4m";  // Underline
    }

    /// Function which replaces the custom ansi mapping strings
    /// with their respective ansi colour codes. This function
    /// is utilised by the [`super::colprint!`] and [`super::colprintln!`]
    /// macros
    /// 
    /// ---
    /// ## Examples
    /// ```
    /// use utils::ansi::replace_cc;
    /// 
    /// let manual = String::from("\x1b[1m\x1b[3m\x1b[32mHello, world!\x1b[0m");
    /// let output = replace_cc(String::from("_BLD_ITL.GRNHello, world!_X"));
    /// assert_eq!(output, manual);
    /// ```
    pub fn replace_cc(s: String) -> String {
        let mut s = s;

        for (from, to) in REPLACE_MAP.iter() {
            s = s.replace(from, to);
        }

        s
    }
}

/// Macro which automatically uses [`ansi::replace_cc`] to
/// replace custom codes with ansi color escape codes and
/// [`print!`]s it to the stdout.
/// 
/// ---
/// # Examples
/// ```
/// use utils::colprint;
/// 
/// colprint!("#CYN.WHT_BLD_ITLHello, world!_X");
/// ```
/// ---
/// # Codes
/// | Text | To  | Color / Style |
/// | ---- | --- | ------------- |
/// | `_X` | \x1b[0m | Clear ANSI formatting |
/// | `_BLD` | `\x1b[1m` | Bold |
/// | `_ITL` | `\x1b[3m` | Italicized |
/// | `_UND` | `\x1b[4m` | Underline |
/// | `.BLK` | `\x1b[30m` | Text Black |
/// | `.RED` | `\x1b[31m` | Text Red |
/// | `.GRN` | `\x1b[32m` | Text Green |
/// | `.YLW` | `\x1b[33m` | Text Yellow |
/// | `.BLU` | `\x1b[34m` | Text Blue |
/// | `.MGT` | `\x1b[35m` | Text Magenta |
/// | `.CYN` | `\x1b[36m` | Text Cyan |
/// | `.WHT` | `\x1b[37m` | Text White |
/// | `#BLK` | `\x1b[40m` | Background Black |
/// | `#RED` | `\x1b[41m` | Background Red |
/// | `#GRN` | `\x1b[42m` | Background Green |
/// | `#YLW` | `\x1b[43m` | Background Yellow |
/// | `#BLU` | `\x1b[44m` | Background Blue |
/// | `#MGT` | `\x1b[45m` | Background Magenta |
/// | `#CYN` | `\x1b[46m` | Background Cyan |
/// | `#WHT` | `\x1b[47m` | Background White |
/// 
#[macro_export]
#[cfg(feature = "ansi")]
macro_rules! colprint {
    () => {
        print!()
    };
    ($($arg:tt)*) => {{
        let txt = utils::ansi::replace_cc(format!($($arg)*));
        print!("{txt}")
    }}
}

/// Macro which automatically uses [`ansi::replace_cc`] to
/// replace custom codes with ansi color escape codes and
/// [`println!`]s it to the stdout.
/// 
/// ---
/// # Examples
/// ```
/// use utils::colprintln;
/// 
/// colprintln!("#CYN.WHT_BLD_ITLHello, world!_X");
/// ```
/// ---
/// # Codes
/// | Text | To  | Color / Style |
/// | ---- | --- | ------------- |
/// | `_X` | \x1b[0m | Clear ANSI formatting |
/// | `_BLD` | `\x1b[1m` | Bold |
/// | `_ITL` | `\x1b[3m` | Italicized |
/// | `_UND` | `\x1b[4m` | Underline |
/// | `.BLK` | `\x1b[30m` | Text Black |
/// | `.RED` | `\x1b[31m` | Text Red |
/// | `.GRN` | `\x1b[32m` | Text Green |
/// | `.YLW` | `\x1b[33m` | Text Yellow |
/// | `.BLU` | `\x1b[34m` | Text Blue |
/// | `.MGT` | `\x1b[35m` | Text Magenta |
/// | `.CYN` | `\x1b[36m` | Text Cyan |
/// | `.WHT` | `\x1b[37m` | Text White |
/// | `#BLK` | `\x1b[40m` | Background Black |
/// | `#RED` | `\x1b[41m` | Background Red |
/// | `#GRN` | `\x1b[42m` | Background Green |
/// | `#YLW` | `\x1b[43m` | Background Yellow |
/// | `#BLU` | `\x1b[44m` | Background Blue |
/// | `#MGT` | `\x1b[45m` | Background Magenta |
/// | `#CYN` | `\x1b[46m` | Background Cyan |
/// | `#WHT` | `\x1b[47m` | Background White |
/// 
#[cfg(feature = "ansi")]
#[macro_export]
macro_rules! colprintln {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let txt = utils::ansi::replace_cc(format!($($arg)*));
        println!("{txt}")
    }}
}