RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--library-path=/mnt/d/CSCD94/rustos_v2/src -C link-arg=--script=kernel.ld" cargo rustc --target=aarch64-unknown-none-softfloat --release
rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img
docker run --privileged -t --rm -v $(pwd):/work/tutorial -w /work/tutorial -i rustembedded/osdev-utils:2021.12 qemu-system-aarch64 -M raspi3b -serial stdio -display none -kernel kernel8.img -S -s