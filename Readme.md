# Rust OS
Rust OS is a custom OS written in Rust for the Raspberry PI 3b+ bcm2837 board.

# Building
To build the kernel you need to first run the following commands
to install the dependencies <br/>
`docker pull rustembedded/osdev-utils:2021.12`
`cargo install cargo-binutils` <br/>
`rustup component add llvm-tools-preview` <br/>
`rustup target add aarch64-unknown-none-softfloat`

In order to build the kernel run the `build.sh` script in the root folder

# Debugging
In order to debug the kernel run the `buildDebug.sh` script in the root folder
In another terminal use gdbmultiarch
run the following commands in gdb

`
set architecture aarch64
`
<br/>
`
file target/aarch64-unknown-none-softfloat/release/kernel
`
<br/>
`
target remote :1234
`

after that you should be able to set debug points in the kernel and run gdb normally

# Features
Currently the feature set is almost comparable to the PINTOS OS it features

- Interrupt Handling for EL1 IRQ interrupts
- Basic Process Scheduler
- Basic Memory Allocator

# In Complete Features
Currently every process operates on a kernel level and there is no process
isolation with virtual memory

- User Process/System Calls (In Progress)
- Virtual Memory
- File System

# Future Work
As of right now the code is a heavily bastardize version of rust that really take advantage of the 
languages memory safe features. The code has works with a lot of raw pointers and has alot
of unsafe function. This is partially due to my unfamiliarity with rust,
so any future work should heavily refactor the source code to be more inline
with the rust way of programming.

# References

- https://github.com/s-matyukevich/raspberry-pi-os
- https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
- https://github.com/rcore-os/rCore
- https://os.phil-opp.com/
