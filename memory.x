MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* qemu-system-arm -machine mps2-an385 has 4MB Flash and 4MB SRAM */
  /* The BIOS has 128K of flash */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  /* Top of DTCM is for the BIOS and the OS */
  RAM : ORIGIN = 0x20078000, LENGTH = 32K
  /* Bottom is for the Application area, but skipping 4K */
  RAM_OS : ORIGIN = 0x20001000, LENGTH = 476K
}

/*
 * Export some symbols to tell the BIOS where the OS RAM is.
 */
_ram_os_start = ORIGIN(RAM_OS);
_ram_os_len = LENGTH(RAM_OS);

_tpa_start = ORIGIN(RAM_OS);
