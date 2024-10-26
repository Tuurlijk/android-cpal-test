# android-cpal-test
Generate sound data in rust and play on android

The library generates a simple one second sound of 440 Hz at half amplitude.

## Running the example rust app

```rust
cargo run --example beep
```
## Cargo ndk

We use [cargo-ndk](https://github.com/bbqsrc/cargo-ndk) to compile Rust projects against the Android NDK without hassle.

You can install it using:

```shell
cargo install cargo-ndk
```

And setup the toolchains you intend to use using:

```shell
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

## Uniffi bindings generator

[Uniffi](https://github.com/mozilla/uniffi-rs) is a multi-language bindings generator for rust written by mozilla.

We follow [the tutorial](https://mozilla.github.io/uniffi-rs/latest/Getting_started.html) using an udl file to expose the lib to Kotlin.

Build the libs with `./build.sh`, then build the android app in Android Studio.


## Reference

* [https://sal.dev/android/intro-rust-android-uniffi/](https://sal.dev/android/intro-rust-android-uniffi/)