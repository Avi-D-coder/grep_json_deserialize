use std::ops::Range;

/// A parser for the output of [grep_printer::JSON](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html).
/// Created to deserialize `ripgrep` `--json` output for [rg_replace](https://github.com/Avi-D-coder/rg_replace).

pub enum Type {
    /// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-begin
    Begin(ArbitraryData),
    End,
    Match,
    Context,
}

/// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-arbitrary-data
pub enum ArbitraryData {
    Text(String),
    Base64(String),
}

/// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-end
pub struct End {
    pub path: ArbitraryData,
    pub binary_offset: Option<isize>, // FIXME I am not sure it's a isize
    pub stats: Stats,
}

/// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-stats
pub struct Stats {
    pub elapsed: Duration,
    pub searches: isize,
    pub searches_with_match: usize,
    pub bytes_searched: usize,
    pub bytes_printed: usize,
    pub matched_lines: usize,
    pub matches: usize,
}

/// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-duration
pub struct Duration {
    pub secs: usize,
    pub nanos: usize,
    pub human: f64,
}

/// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-match
pub struct Match {
    pub path: ArbitraryData,
    pub lines: ArbitraryData,
    pub line_number: Option<usize>,
    pub absolute_offset: isize,
    pub submatches: Vec<Submatch>, //TODO Optimize
}

/// Almost as specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-submatch
/// `match` is deserialized to `matched` because a rust reserves match as a keyword.
/// `start` and `end` are also deserialized to `range`.
pub struct Submatch {
    pub matched: ArbitraryData,
    pub range: Range<usize>,
}

/// As specified in: https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-context
pub struct Context {
    pub path: ArbitraryData,
    pub lines: ArbitraryData,
    pub line_number: Option<usize>,
    pub absolute_offset: isize,
    pub submatches: Vec<Submatch>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unimplemented!();
    }
}
