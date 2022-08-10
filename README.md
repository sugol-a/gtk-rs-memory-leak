GSK render node memory leak test case

To see the bug in action:

```
$ cargo build --features=leak
$ heaptrack target/debug/gsk-memory-leak
```
