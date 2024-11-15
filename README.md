# `godot-rust-macOS-arm64`

* Made following [Godot Rust: Book/Intro/HelloWorld](https://godot-rust.github.io/book/intro/hello-world.html)
* Targets `macOS.arm64`.
* Not planning on actively maintaining this, but wanted to share it.
* Following the `HelloWorld` will net you the same result.

## Godot

* Open `godot`
* Import Project `Rust-Godot/godot/project.godot`

## Rust

* `cd rust/ && cargo build --target aarch64-apple-darwin`

## Notes

* Made following [Godot Rust: Book/Intro/HelloWorld](https://godot-rust.github.io/book/intro/hello-world.html)
* `godot/.godot/extension_list.cfg` - Tells Godot where `librust.gdextension` is for the `Project Tree`.
* `godot/librust.gdextension` - Tells Godot how to interop with `aarch64-apple-darwin/debug/librust.dylib` and other `targets`.

* `rust/target/aarch64-apple-darwin/debug/librust.dylib` - Our compiled `library`.

## OS Caveats
`versions`
```shell
Godot Engine v4.2.1.stable.mono.official.b09f793f5 - https://godotengine.org
Vulkan API 1.2.231 - Forward+ - Using Vulkan Device #0: Apple - Apple M1
```

`librust.gdextension`
```TOML
.
.
.
macos.debug.arm64 =      "res://../rust/target/aarch64-apple-darwin/debug/librust.dylib" # Specific to me.
macos.release.arm64 =    "res://../rust/target/aarch64-apple-darwin/release/librust.dylib" # Specific to me.

```