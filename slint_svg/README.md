```sh
SLINT_LIVE_PREVIEW=1 cargo run --features slint/live-preview
```

На ARMv7

```sh
cross build --target="armv7-unknown-linux-gnueabihf" --release --offline; scp target/armv7-unknown-linux-gnueabihf/release/slint_svg root@target:/root
```
