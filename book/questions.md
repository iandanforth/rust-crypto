

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


I'm sad rust uses environment variables for configuration


------
https://doc.rust-lang.org/book/second-edition/ch06-01-defining-an-enum.html
If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct.

This seems to work just fine

```rust
struct IP {
    v4: (u8, u8, u8, u8),
    v6: String
}
```


The distinction between a struct and an enum is too loose. Their usecases overlap which will lead to user confusion.

#########

The default clause in a match is dumb, using _ isn't clear. The default keyword is much more helpful.

#########

if let should not have been added. A standard if clause and a match cover all required cases. This is just lazy syntax for lazy typists.


#########

Public and private methods arn't super useful. Python would be a much more difficult language to use if you coudln't easily reach inside a module and use inner functions. A developer might think 
that a function should be private but only the user knows what they need.