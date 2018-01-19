

Why doesn't this work?

```rust

let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = a[0..3];

```

```
error[E0277]: the trait bound `[i32]: std::marker::Sized` is not satisfied
  --> src/main.rs:17:9
   |
17 |     let b = a[0..3];
   |         ^ `[i32]` does not have a constant size known at compile-time
```

How do you give it a sized trait if not by specifying a size for the array?

Why is the struct block separate from the impl block? This isn't user friendly and allows code that should be collocated to be visually separated

Also, blech that this allows *multiple* implementation blocks