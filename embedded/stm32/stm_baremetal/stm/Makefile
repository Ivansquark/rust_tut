all:
	mkdir -p build
	cargo build
	cp target/thumbv7em-none-eabihf/debug/stm build/main.elf
	arm-none-eabi-size build/main.elf
	arm-none-eabi-objcopy -O binary build/main.elf build/main.bin
	arm-none-eabi-objdump -D build/main.elf > build/main.lst

release:
	mkdir -p build
	cargo build --release
	cp target/thumbv7em-none-eabihf/release/stm build/main.elf
	arm-none-eabi-size build/main.elf
	arm-none-eabi-objcopy -O binary build/main.elf build/main.bin
	arm-none-eabi-objdump -D build/main.elf > build/main.lst

clean:
	rm -rf build

load:
	openocd -f lib/stlink.cfg -f lib/stm32f4x.cfg -c "program build/main.bin \
	verify reset exit 0x08000000"
