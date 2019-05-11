elf := target/riscv64imac-unknown-none-elf/release/rv64-kernel
bin := target/riscv64imac-unknown-none-elf/release/bin
objdump := cargo objdump -- -arch-name=riscv64
objcopy := cargo objcopy -- --binary-architecture=riscv64

env:
	rustup target add riscv64imac-unknown-none-elf
	rustup component add llvm-tools-preview
	
opensbi:
	wget https://github.com/riscv/opensbi/releases/download/v0.3/opensbi-0.3-rv64-bin.tar.xz
	tar -xf opensbi-0.3-rv64-bin.tar.xz
	mv opensbi-0.3-rv64-bin opensbi

run: env build opensbi
	qemu-system-riscv64 -M virt -m 256M -nographic -serial mon:stdio \
		-kernel opensbi/platform/qemu/virt/firmware/fw_jump.elf \
		-device loader,file=$(bin),addr=0x80200000

build: bin

bin: $(elf)
	$(objcopy) -O binary $(elf) $(bin)

elf:
	cargo xbuild --target riscv64imac-unknown-none-elf --release

asm:
	$(objdump) -d $(elf) | less

sym:
	$(objdump) -t $(elf) | less

$(bin): bin

$(elf): elf
