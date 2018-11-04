#### Deserialization for the output of [grep_printer::JSON](https://docs.rs/grep-printer/0.1.1/grep_printer/struct.JSON.html).
Created to deserialize `ripgrep` `--json` output for [rg_replace](https://github.com/Avi-D-coder/rg_replace).
Powered by [serde](https://serde.rs/).

Cargo.toml:
```toml
grep_json_deserialize = "0.1.1"
serde_json = "1.0.32"
```
```
serde_json::from_str(&line).unwrap()
```
