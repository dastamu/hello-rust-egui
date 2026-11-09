# hello-rust-egui
> Rust GUI example

![Image 1](screenshot.png) ![Image 2](screenshot2.png)

## Deps
```sh
#@ AL10
sudo dnf install rust cargo
# Size: 283MB

#@ Win11 & MINGW64
./rustup-init.exe
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
# Size: 3.44GB
```

## Run
```sh
cargo build --release
cargo run --release
cargo clean
```
