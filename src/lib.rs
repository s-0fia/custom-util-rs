pub use util_derive::{Add, Sub, Mul, Div, PartialOps};
// extern crate util_derive;
pub use chrono;

#[macro_export]
macro_rules! perf_time {
    ($f: ident) => { perf_time!(1_000_000, $f) }; // Function only, 10⁶ times
    ($iters: expr, $f: ident) => { // Number of iterations and function name
        {
            let r = 1..=$iters;

            let start = std::time::Instant::now();

            for _ in r {
                let _ = $f();
            }

            start.elapsed()
        }
    };
    ($data: expr, $f: expr) => { // Fuzz data and function name
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

#[macro_export]
macro_rules! timeprintln {
    () => { 
        timeprintln!(as "[%H:%M:%S]")
    };
    (as $f:expr) => {
        println!("{}", utils::chrono::Local::now().format($f))
    };
    (as $f:expr; $($arg:tt)*) => { // this has to be before
        {
            print!("{} ", utils::chrono::Local::now().format($f));
            println!($( $arg )*)
        }
    };
    ($($arg:tt)*) => { // this
        timeprintln!(as "[%H:%M:%S]"; $( $arg )*)
    };
}
