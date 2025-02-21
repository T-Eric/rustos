TARGET := riscv64gc-unknown-none-elf
RUN_MODE := release
ORIGIN := target/$(TARGET)/$(RUN_MODE)/rustos
BIN := target/$(TARGET)/$(RUN_MODE)/os.bin

PLATFORM := qemu-system-riscv64
MACHINE := virt
BOOTLOADER := bootloader/rustsbi-qemu.bin

gen_bin:
	cargo build
	-cargo run --release
	rust-objcopy --strip-all $(ORIGIN) -O binary $(BIN)
	stat $(BIN)

run:
	$(PLATFORM) -machine $(MACHINE) -nographic -bios $(BOOTLOADER) -device loader,file=$(BIN),addr=0x80200000

run_debug:
	$(PLATFORM) -machine $(MACHINE) -nographic -bios $(BOOTLOADER) -device loader,file=$(BIN),addr=0x80200000 -gdb tcp::3456 -S

.PHONY: gen_bin run run_debug