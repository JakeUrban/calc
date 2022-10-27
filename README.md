## calc

A simple calculator CLI. 

```bash
$ calc 1 + 1
result: 2
```

Only support integers. Legal operations:
- addition (`+`)
- subtraction (`-`)
- multiplication (`*`)
    - make sure to escape this character (ex. `calc 1 \* 2`)
- division (`/`)
- exponent (`**`)
    - make sure to escape this character (ex. `calc 2 \*\* 3`)

### install

```
$ cargo build
$ mv ./target/debug/calc /usr/local/bin/
```
