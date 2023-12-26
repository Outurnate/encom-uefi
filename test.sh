#!/bin/sh
cargo build --target x86_64-unknown-uefi
cp /usr/share/OVMF/x64/OVMF_CODE.fd .
cp /usr/share/OVMF/x64/OVMF_VARS.fd .
mkdir -p esp/efi/boot
cp target/x86_64-unknown-uefi/debug/uefi-cheese.efi esp/efi/boot/bootx64.efi
rm esp/NvVars
qemu-system-x86_64 \
  -nodefaults \
  -device virtio-rng-pci \
  -boot menu=on,splash-time=0 \
  -fw_cfg name=opt/org.tianocore/X-Cpuhp-Bugcheck-Override,string=yes \
  -machine q35 \
  -smp 4 \
  -m 256M \
  -vga std \
  -enable-kvm \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
  -drive format=raw,file=fat:rw:esp