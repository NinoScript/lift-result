# lift_result

Lifts a fallible function that fails with error E1, into one that fails with error E2.

```rust
result
    .map_err(Error::from)
    .and_then(lift(failable))
```

## Backstory

I had my own `Error` type, turning another result into mine was easy and nice

```rust
result.map_err(Error::from)
```

But when I wanted to apply a fallible function via `.and_then`, I had to do this:

```rust
result.and_then(|x| failable(x).map_err(|e| e.into()))
```

And I didn't like that:
* too verbose
* not very readable
* the compiler should be able to know how to do that

So I wrote this library.
Yay, programming! 🎉

## Special thanks
Thank you [cargo-readme](https://github.com/livioribeiro/cargo-readme) for generating this README for me.


License: MIT OR Apache-2.0
