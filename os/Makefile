TARGET := riscv64gc-unknown-none-elf
RUN_MODE := release
ORIGIN := target/$(TARGET)/$(RUN_MODE)/os
BIN := target/$(TARGET)/$(RUN_MODE)/os.bin
ENTRY_ADDR := 0x80000000

PLATFORM := qemu-system-riscv64
MACHINE := virt
#BOOTLOADER := ../bootloader/rustsbi-qemu.bin
BOOTLOADER := none
GDB := riscv64-unknown-elf-gdb

gen_bin:
	cargo build
	-cargo run --release
	rust-objcopy --strip-all $(ORIGIN) -O binary $(BIN)
	stat $(BIN)

run:
	$(PLATFORM) -machine $(MACHINE) -nographic -bios $(BOOTLOADER) -device loader,file=$(BIN),addr=$(ENTRY_ADDR)

run_debug:
	$(PLATFORM) -machine $(MACHINE) -nographic -bios $(BOOTLOADER) -device loader,file=$(BIN),addr=$(ENTRY_ADDR) -gdb tcp::3456 -S

start_gdb:
	$(GDB) \ -ex 'file $(ORIGIN)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:3456'
.PHONY: gen_bin run run_debug start_gdb
