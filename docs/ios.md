# iOS

The oldest supported device is an iPhone 5S on iOS 12.5, an A7 GPU. Everything here exists
to keep that device working. Nothing is cosmetic, and each knob was found by a crash.

## Logs

iOS drops a launched app's stdout and stderr, so `println!`, the fern chain and the default
panic hook all go nowhere. `ios_log.rs` routes three things through `NSLog`, which reaches
the device system log:

- the fern chain, so `debug!` and friends show up,
- a panic hook, or a panic leaves only `abort()` in the crash report with no message,
- an `objc_setExceptionPreprocessor`, which logs an Objective-C exception at the throw.

The last one matters more than it looks. An ObjC exception that unwinds through an
`extern "C"` frame, a UIKit callback block for instance, makes Rust abort with `panic in a
function that cannot unwind`, reporting neither the reason nor the throw site. A
`panic in a function that cannot unwind` with no `panicked at` line before it is always a
foreign exception, never a Rust panic.

Read the log with `idevicesyslog -u <udid>`, unfiltered. Its `-m` filter silently drops
matching lines.

## Two version settings that look alike

- `IPHONEOS_DEPLOYMENT_TARGET` is a **compile time** value. `objc2::available!` reads it
  through `option_env!` and folds to a constant `true` whenever the target already meets
  the version being checked, which deletes the runtime guard. Setting it to 13.0 made
  wgpu's `available!(ios = 13.0)` vanish and sent `supportsFamily:` to the A7, which has
  no such selector. `make build-ios` pins 12.0. Never raise it to or above a version the
  code guards against.
- `.cargo/config.toml` `-Wl,-platform_version,ios,13.0,13.0` is a **linker** flag and does
  not affect `available!`. It stays at 13.0 because `aws-lc-sys` needs `__chkstk_darwin`,
  an iOS 13 symbol. Lowering it breaks the link of the dylib crates.

`cargo build --lib` links nothing, so it cannot reproduce a link error that
`make build-ios` hits. Check with the Makefile target.

## Weak linked CoreGraphics

`OTHER_LDFLAGS` in the Xcode project carries `-Wl,-weak_framework,CoreGraphics`. wgpu names
`kCGColorSpaceExtendedDisplayP3` and the two `kCGColorSpaceITUR_2100` constants, all iOS
14, and a Rust `extern` static has no availability metadata, so rustc always emits a strong
reference and dyld aborts before `main` on an older device. Weak linking makes the missing
constants NULL, which is safe because the surface only ever asks for sRGB. The deployment
target cannot fix this.

## wgpu fork

`wgpu` and `wgpu_text` come from forks pinned by git rev in the root `Cargo.toml`. The wgpu
fork guards `wantsExtendedDynamicRangeContent`, an iOS 16 property that upstream messages
with no availability check, which breaks every iOS below 16. `wgpu_text` points at the same
fork because a crates.io copy would give two `wgpu` crates whose types do not interchange.
Do not swap either back to crates.io without checking on an old device first.

## A7 limits

The A7 reports missing downlevel flags: indirect execution, base vertex and cube array
texturing. Rendering works, but a UI test that leans on those paths will fail there and
pass everywhere else.
