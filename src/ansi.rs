pub const REPLACE_MAP: [(&str, &str); 20] = [
    ("_X", self::X), ("_BLD", style::BLD), ("_ITL", style::ITL), ("_UND", style::UND),
    (".BLK", BLK), (".RED", RED), (".GRN", GRN), (".YLW", YLW), (".BLU", BLU), (".MGT", MGT), (".CYN", CYN), (".WHT", WHT),
    ("#BLK", BBLK), ("#RED", BRED), ("#GRN", BGRN), ("#YLW", BYLW), ("#BLU", BBLU), ("#MGT", BMGT), ("#CYN", BCYN), ("#WHT", BWHT)
];

// Denoted by a _
pub const X:       &str = "\x1b[0m";  // Clear code

pub const BLK: &str = "\x1b[30m"; // Black
pub const RED: &str = "\x1b[31m"; // Red
pub const GRN: &str = "\x1b[32m"; // Green
pub const YLW: &str = "\x1b[33m"; // Yellow
pub const BLU: &str = "\x1b[34m"; // Blue
pub const MGT: &str = "\x1b[35m"; // Magenta
pub const CYN: &str = "\x1b[36m"; // Cyan
pub const WHT: &str = "\x1b[37m"; // White

pub mod f_color { // Denoted by a .

    #[macro_export]
    macro_rules! set_frgb {
        () => {
            print!("{}", super::X)
        };
        ($r:tt, $g:tt, $b:tt) => {
            print!("\x1b[38;2;{};{};{}m", $r, $g, $b)
        }
    }

    pub fn rgb([r, g, b]: [usize; 3]) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }
}

pub const BBLK: &str = "\x1b[40m"; // Black
pub const BRED: &str = "\x1b[41m"; // Red
pub const BGRN: &str = "\x1b[42m"; // Green
pub const BYLW: &str = "\x1b[43m"; // Yellow
pub const BBLU: &str = "\x1b[44m"; // Blue
pub const BMGT: &str = "\x1b[45m"; // Magenta
pub const BCYN: &str = "\x1b[46m"; // Cyan
pub const BWHT: &str = "\x1b[47m"; // White

pub mod b_color { // Denoted by a #

    #[macro_export]
    macro_rules! set_brgb {
        () => {
            print!("{}", super::X)
        };
        ($r:tt, $g:tt, $b:tt) => {
            print!("\x1b[48;2;{};{};{}m", $r, $g, $b)
        }
    }

    pub fn rgb([r, g, b]: [usize; 3]) -> String {
        format!("\x1b[48;2;{};{};{}m", r, g, b)
    }
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

pub mod cursor {
    pub const HOME:  &str = "\x1b[H";
    pub const LN_UP: &str = "\x1bM";

    // moves cursor to home position (0, 0)
    pub fn reset() {
        print!("{HOME}");
    }

    // moves cursor to line #, column #
    pub fn set_pos([line, column]: [usize; 2]) {
        print!("\x1b[{line};{column}H");
    }

    // moves cursor up # lines
    pub fn up(lines: usize) {
        print!("\x1b[{lines}A");
    }

    // moves cursor down # lines
    pub fn down(lines: usize) {
        print!("\x1b[{lines}B");
    }

    // moves cursor right # columns
    pub fn right(columns: usize) {
        print!("\x1b[{columns}C");
    }

    // moves cursor left # columns
    pub fn left(columns: usize) {
        print!("\x1b[{columns}D");
    }

    // moves cursor to beginning of next line, # lines down
    pub fn new_lines_down(lines: usize) {
        print!("\x1b[{lines}E");
    }

    // moves cursor to beginning of previous line, # lines up
    pub fn lines_up(lines: usize) {
        print!("\x1b[{lines}F");
    }

    // moves cursor to column #
    pub fn set_column(column: usize) {
        print!("\x1b[{column}G");
    }

    // moves cursor one line up and scrolls if needed
    pub fn one_line_up() {
        print!("{LN_UP}");
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