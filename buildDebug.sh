RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--library-path=/mnt/d/CSCD94/rustos_v2/src -C link-arg=--script=kernel.ld" cargo rustc --target=aarch64-unknown-none-softfloat --release
rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img
qemu-system-aarch64 -M raspi3b -cpu cortex-a53 -kernel ./kernel8.img -serial null -serial stdio -display none -s -S