# Keymini Rust firmware

Install the complete toolchain and utils:

```shell
curl https://sh.rustup.rs -sSf | sh
# restart terminal to update the environment
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install cargo-binutils
sudo apt-get install dfu-util
```

Compile:

```shell
cd firmware
cargo objcopy --bin keymini --release -- -O binary firmware.bin
```

To flash using dfu-util, launch it with:
```shell
dfu-util -w -d 0483:df11 -a 0 -s 0x08000000:leave -D firmware.bin
```

Then, put the board in DFU mode by:
 * Putting the keyboard in DFU mode as defined in the firmware. By
   default, it’s the leftmost key on the function layer. So, press
   simultaneously the 2 middle thumb keys, and then tap the leftmost
   thumb key.
 * Or (for the first time, or in case of buggy firmware) by shorting
   the 2 pads under the board (with a piece of aluminium foil for
   example, pressing hard) and plug USB while doing the connection.

Upload should begin as soon as the computer detects the board in
DFU mode.

The first time, if the write fail, your flash might be protected. To
unprotect:

```shell
dfu-util -d 0483:df11 -a 0 -s 0x08000000:force:unprotect -D firmware.bin
```
