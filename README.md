# Rust -Zextra-link-arg Test

This repo provides an example of how `-Zextra-link-arg` can work in Rust.

It consists of three packages:

* `lib-maybe-link`: A library that may or may not set linker flags
* `without-linker`: Uses `lib-maybe-link` without requesting linker flags
* `attempt-linker`: Uses `lib-maybe-link` with linker flags

# Usage

Install an embedded target that supports linker flags. For example:

```
rustup +nightly target add riscv32imac-unknown-none-elf
```

Next, build `without-linker`, and observe how it builds:

```
[10:15:54 AM] D:/Code/linker-test> cargo +nightly -Zextra-link-arg build -p without-linker --target riscv32imac-unknown-none-elf
   Compiling lib-maybe-link v0.1.0 (D:\Code\linker-test\lib-maybe-link)
   Compiling without-linker v0.1.0 (D:\Code\linker-test\without-linker)
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
[10:15:57 AM] D:/Code/linker-test>
```

Attempt to build `attempt-linker`, and observe that it fails because it's picked up the nonexistent linker flag:

```
[10:21:50 AM] D:/Code/linker-test> cargo +nightly -Zextra-link-arg build -p attempt-linker --target riscv32imac-unknown-none-elf
   Compiling lib-maybe-link v0.1.0 (D:\Code\linker-test\lib-maybe-link)
   Compiling attempt-linker v0.1.0 (D:\Code\linker-test\attempt-linker)
error: linking with `rust-lld` failed: exit code: 1
  |
  = note: "rust-lld" "-flavor" "gnu" "-L" "C:\\Users\\smcro\\.rustup\\toolchains\\nightly-x86_64-pc-windows-gnu\\lib\\rustlib\\riscv32imac-unknown-none-elf\\lib" "D:\\Code\\linker-test\\target\\riscv32imac-unknown-none-elf\\debug\\deps\\attempt_linker-e325983111c6bb57.35ff07kpanugha7m.rcgu.o" "-o" "D:\\Code\\linker-test\\target\\riscv32imac-unknown-none-elf\\debug\\deps\\attempt_linker-e325983111c6bb57" "--gc-sections" "-L" "D:\\Code\\linker-test\\target\\riscv32imac-unknown-none-elf\\debug\\deps" "-L" "D:\\Code\\linker-test\\target\\debug\\deps" "-L" "C:\\Users\\smcro\\.rustup\\toolchains\\nightly-x86_64-pc-windows-gnu\\lib\\rustlib\\riscv32imac-unknown-none-elf\\lib" "-Bstatic" "D:\\Code\\linker-test\\target\\riscv32imac-unknown-none-elf\\debug\\deps\\liblib_maybe_link-7e3515822fca08dd.rlib" "C:\\Users\\smcro\\.rustup\\toolchains\\nightly-x86_64-pc-windows-gnu\\lib\\rustlib\\riscv32imac-unknown-none-elf\\lib\\librustc_std_workspace_core-f8dea4d885a94b85.rlib" "C:\\Users\\smcro\\.rustup\\toolchains\\nightly-x86_64-pc-windows-gnu\\lib\\rustlib\\riscv32imac-unknown-none-elf\\lib\\libcore-39eb6a0559508a94.rlib" "C:\\Users\\smcro\\.rustup\\toolchains\\nightly-x86_64-pc-windows-gnu\\lib\\rustlib\\riscv32imac-unknown-none-elf\\lib\\libcompiler_builtins-ce3fc8c77ec9a399.rlib" "--passing-args-does-work" "-Bdynamic"
  = note: rust-lld: error: unknown argument '--passing-args-does-work'


error: aborting due to previous error

error: could not compile `attempt-linker`

To learn more, run the command again with --verbose.
[10:22:07 AM] D:/Code/linker-test>
```

Finally, build `without-linker` and observe that the change to the `maybe-link` library feature flags don't force a rebuild:

```
[10:16:48 AM] D:/Code/linker-test> cargo +nightly -Zextra-link-arg build -p without-linker --target riscv32imac-unknown-none-elf
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
[10:17:42 AM] D:/Code/linker-test>
```
