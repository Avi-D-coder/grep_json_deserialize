extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate base64;

// use std::ops::Range;

// TODO Use `&str` where possible

/// A parser for the output of [grep_printer::JSON](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html).
/// Created to deserialize `ripgrep` `--json` output for [rg_replace](https://github.com/Avi-D-coder/rg_replace).
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "data")]
pub enum Type {
    /// As specified in: [message-begin](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-begin).
    Begin { path: ArbitraryData },
    /// As specified in: [message-end](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-end).
    End {
        path: ArbitraryData,
        binary_offset: Option<isize>, // FIXME I am not sure it's a isize
        stats: Stats,
    },
    /// As specified in: [message-match](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-match).
    Match {
        path: ArbitraryData,
        lines: ArbitraryData,
        line_number: Option<usize>,
        absolute_offset: isize,
        submatches: Vec<Submatch>, //TODO Optimize
    },
    /// As specified in: [message-context](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#message-context).
    Context {
        path: ArbitraryData,
        lines: ArbitraryData,
        line_number: Option<usize>,
        absolute_offset: isize,
        submatches: Vec<Submatch>,
    },
}

/// As specified in: [object-arbitrary-data](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-arbitrary-data).
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum ArbitraryData {
    Text { text: String },
    Base64 { bytes: String },
}

impl ArbitraryData {
    pub fn lossy_utf8(&self) -> String {
        match self {
            ArbitraryData::Text { text } => text.to_owned(),
            ArbitraryData::Base64 { bytes } => {
                String::from_utf8_lossy(base64::decode(bytes).unwrap().as_slice()).to_string()
            }
        }
    }
}

/// As specified in: [object-stats](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-stats).
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Stats {
    pub elapsed: Duration,
    pub searches: isize,
    pub searches_with_match: usize,
    pub bytes_searched: usize,
    pub bytes_printed: usize,
    pub matched_lines: usize,
    pub matches: usize,
}

/// As specified in: [object-duration](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-duration).
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Duration {
    pub secs: usize,
    pub nanos: usize,
    pub human: String,
}

/// Almost as specified in: [object-submatch](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#object-submatch).
/// `match` is deserialized to `matched` because a rust reserves match as a keyword.
// TODO `start` and `end` are also deserialized to `range`.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename = "submatch")]
pub struct Submatch {
    #[serde(rename = "match")]
    pub matched: ArbitraryData,
    pub start: usize,
    pub end: usize,
    // pub range: Range<usize>, TODO
}

#[cfg(test)]
mod tests {
    // tests based on [`grep_printer` example output](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html#example)

    use crate::{ArbitraryData::*, Type::*, *};

    #[test]
    fn arbitrarydata() {
        let json = r#"{"text":"/home/andrew/sherlock"}"#;
        assert_eq!(
            Text {
                text: "/home/andrew/sherlock".to_owned()
            },
            serde_json::from_str(json).unwrap()
        )
    }

    #[test]
    fn begin_deserialize() {
        let json = r#"{"type":"begin","data":{"path":{"text":"/home/andrew/sherlock"}}}"#;
        assert_eq!(
            Begin {
                path: Text {
                    text: "/home/andrew/sherlock".to_owned()
                }
            },
            serde_json::from_str(json).unwrap()
        );
    }

    #[test]
    fn end_deserialize() {
        let json=r#"{"type":"end","data":{"path":{"text":"/home/andrew/sherlock"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":36296,"human":"0.0000s"},"searches":1,"searches_with_match":1,"bytes_searched":367,"bytes_printed":1151,"matched_lines":2,"matches":2}}}"#;
        assert_eq!(
            End {
                path: Text {
                    text: "/home/andrew/sherlock".to_owned()
                },
                binary_offset: None,
                stats: Stats {
                    elapsed: Duration {
                        secs: 0,
                        nanos: 36296,
                        human: "0.0000s".to_owned()
                    },
                    searches: 1,
                    searches_with_match: 1,
                    bytes_searched: 367,
                    bytes_printed: 1151,
                    matched_lines: 2,
                    matches: 2
                }
            },
            serde_json::from_str(json).unwrap()
        );
    }

    #[test]
    fn match_deserialize() {
        let json=r#"{"type":"match","data":{"path":{"text":"/home/andrew/sherlock"},"lines":{"text":"but Doctor Watson has to have it taken out for him and dusted,\n"},"line_number":5,"absolute_offset":258,"submatches":[{"match":{"text":"Watson"},"start":11,"end":17}]}}"#;
        assert_eq!(
            Match {
                path: Text {
                    text: "/home/andrew/sherlock".to_owned()
                },
                lines: Text {
                    text: "but Doctor Watson has to have it taken out for him and dusted,\n"
                        .to_owned()
                },
                line_number: Some(5),
                absolute_offset: 258,
                submatches: vec![Submatch {
                    matched: Text {
                        text: "Watson".to_owned()
                    },
                    start: 11,
                    end: 17
                }],
            },
            serde_json::from_str(json).unwrap()
        )
    }

    #[test]
    fn content_deserialize() {
        let json = r#"{"type":"context","data":{"path":{"text":"/home/andrew/sherlock"},"lines":{"text":"can extract a clew from a wisp of straw or a flake of cigar ash;\n"},"line_number":4,"absolute_offset":193,"submatches":[]}}"#;
        assert_eq!(
            Context {
                path: Text {
                    text: "/home/andrew/sherlock".to_owned()
                },
                lines: Text {
                    text: "can extract a clew from a wisp of straw or a flake of cigar ash;\n"
                        .to_owned()
                },
                line_number: Some(4),
                absolute_offset: 193,
                submatches: vec![],
            },
            serde_json::from_str(json).unwrap()
        )
    }

}
