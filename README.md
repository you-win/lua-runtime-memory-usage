Compares memory usage between Lua runtimes (running via [mlua](https://github.com/mlua-rs/mlua)).

```bash
$ cargo run --release --features luajit
   Compiling lua-runtime-test v0.1.0 (C:\Users\theaz\dev\lua-runtime-test)
    Finished `release` profile [optimized] target(s) in 0.45s
     Running `target\release\lua-runtime-test.exe`
current mem usage (kb): 0
current mem usage (kb): 45.623047
```

```bash
$ cargo run --release --features luau
   Compiling lua-runtime-test v0.1.0 (C:\Users\theaz\dev\lua-runtime-test)
    Finished `release` profile [optimized] target(s) in 0.44s
     Running `target\release\lua-runtime-test.exe`
current mem usage (kb): 0
current mem usage (kb): 352.29297
```

```bash
$ cargo run --release --features lua54
   Compiling lua-runtime-test v0.1.0 (C:\Users\theaz\dev\lua-runtime-test)
    Finished `release` profile [optimized] target(s) in 0.37s
     Running `target\release\lua-runtime-test.exe`
current mem usage (kb): 0
current mem usage (kb): 25.739258
```
