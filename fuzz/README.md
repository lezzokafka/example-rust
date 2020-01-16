How to run:

- Add dependency

```bash
cargo afl build
```
Then

```bash
cargo afl fuzz -i in -o out target/debug/fuzz_parse_complex
```