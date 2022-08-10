GSK render node memory leak test case

In reference to [gtk-rs#1091](https://github.com/gtk-rs/gtk4-rs/issues/1091)

To see the bug in action:

```
$ cargo build --features=leak
$ heaptrack target/debug/gsk-memory-leak
```
