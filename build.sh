cargo build --target riscv64gc-unknown-none-elf --release

rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/wfi ./wfi.bin

rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/ebreak ./ebreak.bin

dd if=/dev/zero of=./apps.bin bs=1M count=32

app_num=2
printf '%02x' $app_num | xxd -r -p | dd of=apps.bin conv=notrunc bs=1 seek=0
app1_size=$(stat -c%s ./wfi.bin)
printf '%04x' $app1_size | xxd -r -p | dd of=apps.bin conv=notrunc bs=1 seek=1
dd if=./wfi.bin of=./apps.bin conv=notrunc bs=1 seek=3
app2_size=$(stat -c%s ./ebreak.bin)
printf '%04x' $app2_size | xxd -r -p | dd of=apps.bin conv=notrunc bs=1 seek=9
dd if=./ebreak.bin of=./apps.bin conv=notrunc bs=1 seek=11


mkdir -p ../arceos/payload
mv ./apps.bin ../arceos/payload/apps.bin
