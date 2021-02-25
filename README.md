# russel

An incremental, scheme-targetted successor to [rusl](https://github.com/samrat/rusl).

# execute

```bash
cargo run > russel.s
clang -m64 -fomit-frame-pointer russel.s driver.c
./a.out
```
