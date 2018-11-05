#### Deserialization for the output of [grep_printer::JSON](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html).
Created to deserialize `ripgrep` `--json` output for [rg_replace](https://github.com/Avi-D-coder/rg_replace).
Powered by [serde](https://serde.rs/).

#### Cargo.toml:

```toml
[dependencies]
grep_json_deserialize = "0.1.2"
serde_json = "1.0.32"
```

#### main.rs
```rs
use std::process::Command;

extern crate grep_json_deserialize as deserialize;
use deserialize::{ArbitraryData::*, Type::*, *};

fn main() {
    // grep_json always puts out valid UTF-8
    // let out = String::from_utf8(Command::new("rg").arg("main").output().unwrap().stdout).unwrap();
    let out = r#"{"type":"begin","data":{"path":{"text":"src/main.rs"}}}
{"type":"match","data":{"path":{"text":"src/main.rs"},"lines":{"text":"fn main() {\n"},"line_number":6,"absolute_offset":129,"submatches":[{"match":{"text":"main"},"start":3,"end":7}]}}
{"type":"match","data":{"path":{"text":"src/main.rs"},"lines":{"text":"    let out = String::from_utf8(Command::new(\"rg\").arg(\"main\").output().unwrap().stdout).unwrap();\n"},"line_number":8,"absolute_offset":186,"submatches":[{"match":{"text":"main"},"start":56,"end":60}]}}
{"type":"end","data":{"path":{"text":"src/main.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":12208,"human":"0.000012s"},"searches":1,"searches_with_match":1,"bytes_searched":447,"bytes_printed":521,"matched_lines":2,"matches":2}}}
{"data":{"elapsed_total":{"human":"0.002971s","nanos":2971130,"secs":0},"stats":{"bytes_printed":521,"bytes_searched":447,"elapsed":{"human":"0.000012s","nanos":12208,"secs":0},"matched_lines":2,"matches":2,"searches":1,"searches_with_match":1}},"type":"summary"}"#;

    let deserialized: Vec<Type> = out
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    assert_eq!(deserialized,
    vec![
    Begin {
        path: Text {
        text: "src/main.rs".to_owned()
    }
    },
    Match {
        path: Text {
            text: "src/main.rs".to_owned()
        }, lines: Text {
            text: "fn main() {\n".to_owned()
        }, line_number: Some(6), absolute_offset: 129,
            submatches: vec![SubMatch {
                matched: Text {
                    text: "main".to_owned()
                }, start: 3, end: 7 }]
    },
    Match {
        path: Text {
            text: "src/main.rs".to_owned()
        }, lines: Text {
            text: "    let out = String::from_utf8(Command::new(\"rg\").arg(\"main\").output().unwrap().stdout).unwrap();\n".to_owned()
        }, line_number: Some(8),
            absolute_offset: 186,
            submatches: vec![SubMatch { matched: Text { text: "main".to_owned()
            },
                start: 56,
                end: 60
            }]
    },
    End {
        path: Text {
            text: "src/main.rs".to_owned()
        },
        binary_offset: None,
        stats: Stats {
            elapsed: Duration {
                secs: 0,
                nanos: 12208,
                human: "0.000012s".to_owned()
            }, searches: 1,
            searches_with_match: 1,
            bytes_searched: 447,
            bytes_printed: 521,
            matched_lines: 2,
            matches: 2 } },
            Summary {
                elapsed_total:
                    Duration {
                        secs: 0,
            nanos: 2971130,
            human: "0.002971s".to_owned() },
            stats: Stats {
                elapsed:
                    Duration {
                        secs: 0,
            nanos: 12208,
            human: "0.000012s".to_owned() },
            searches: 1,
            searches_with_match: 1,
            bytes_searched: 447,
            bytes_printed: 521,
            matched_lines: 2,
            matches: 2 } }]);
}
```
