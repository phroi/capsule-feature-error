[Configuration flags](https://doc.rust-lang.org/std/macro.cfg.html) are not forwarded to rust compiler. To test this I created this repo. The following test script:

``` rust
use crate::error::Error;
use core::result::Result;

const RESULT: Result<(), Error> = if cfg!(myfeature) {
    Ok(())
} else {
    panic!("myfeature not found in command line args");
};

pub fn main() -> Result<(), Error> {
    return RESULT;
}
```

Compiled with the following command:

``` sh
capsule build --release -- --features myfeature
```

Or with:

``` sh
capsule build --release
```

Results in the same kind error:

``` bash
$ capsule build --release -- --features myfeature
Building contract capsule-feature-error
RUSTFLAGS=--remap-path-prefix=/home/user=~ --remap-path-prefix=/home/user/capsule-feature-error=
$ cross build -p capsule-feature-error --release --features myfeature
   Compiling capsule-feature-error v0.1.0 (/home/user/capsule-feature-error/contracts/capsule-feature-error)
error[E0080]: evaluation of constant value failed
 --> contracts/capsule-feature-error/src/entry.rs:7:5
  |
7 |     panic!("myfeature not found in command line args");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'myfeature not found in command line args', contracts/capsule-feature-error/src/entry.rs:7:5
  |
  = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0080`.
error: could not compile `capsule-feature-error` due to previous error
error: command exited with non-zero code `cross build -p capsule-feature-error --release --features myfeature`: 101
```

Keep up the great work :muscle:
Phroi

PS: inverting `Ok(())` and `panic!("myfeature not found in command line args");` compile just fine.
