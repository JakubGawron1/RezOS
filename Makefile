.PHONY: clean run

ASM = nasm
ASM_FORMAT = bin
CARGO = cargo
EMU = qemu-system-x86_64
EMU_ARGS = -nographic

build/RezOS.bin: build/boot.bin build/mkfs.exe build/kernel.bin
	build/mkfs.exe -b build/boot.bin -o $@

build/mkfs.exe: mkfs/* Cargo.toml
	$(CARGO) build --bin mkfs --release
	cp target/release/mkfs $@

build/boot.bin: boot/* boot/real/io/*
	$(ASM) -f $(ASM_FORMAT) boot/main.asm -o $@

clean:
	rm -f build/*

run: build/RezOS.bin
	$(EMU) $(EMU_ARGS) build/RezOS.bin

build/kernel:
	echo "no kernel for you trololol" > $@