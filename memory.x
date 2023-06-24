MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* qemu-system-arm -machine mps2-an385 has 4MB Flash and 4MB SRAM */
  /* The BIOS has 128K of flash */
  FLASH : ORIGIN = 0x00000000, LENGTH = 4M
  /* The second 2M of RAM is for the BIOS */
  RAM : ORIGIN = 0x20200000, LENGTH = 2M
  /* We leave the first 2M of RAM for the OS, skipping 4K */
  RAM_OS : ORIGIN = 0x20001000, LENGTH = 2044K
}

/*
 * Export some symbols to tell the BIOS where the OS RAM is.
 */
_ram_os_start = ORIGIN(RAM_OS);
_ram_os_len = LENGTH(RAM_OS);

_tpa_start = ORIGIN(RAM_OS);
