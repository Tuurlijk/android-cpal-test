#!/usr/bin/env sh

#cargo install cargo-ndk

#rustup target add \
#    x86_64-linux-android \
#    i686-linux-android \
#    armv7-linux-androideabi \
#    aarch64-linux-android

cargo build --release

cargo ndk -o ./app/src/main/jniLibs \
  --manifest-path ./Cargo.toml \
  -t x86 \
  -t x86_64 \
  build --release

#  -t armeabi-v7a \
#  -t arm64-v8a \
#  -t x86_64 \

mkdir -p ./app/src/main/java/com/example/androidcpaltest/rust/

cargo run \
  --bin uniffi-bindgen \
  generate src/androidcpaltest.udl \
  --language kotlin \
  -o ./app/src/main/java/com/example/androidcpaltest/rust/
