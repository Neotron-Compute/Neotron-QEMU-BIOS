MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* The MPS3-AN547 boots from 512K of ITCM SRAM at 0x0000_0000 */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  /* We have 512K of DTCM at 0x2000_0000. We use 32K at the top for the BIOS and OS stuff. */
  RAM : ORIGIN = 0x20078000, LENGTH = 32K
  /*
   * This is for the TPA, but skipping the first 4K (to match
   * other platforms)
   */
  RAM_OS : ORIGIN = 0x20001000, LENGTH = 476K

  FPGA_SRAM: ORIGIN = 0x01000000, LENGTH = 2M

  DDR4_SDRAM: ORIGIN = 0x60000000, LENGTH = 2048M
}

/*
 * Export some symbols to tell the BIOS where the OS RAM is.
 */
_ram_os_start = ORIGIN(RAM_OS);
_ram_os_len = LENGTH(RAM_OS);

_tpa_start = ORIGIN(RAM_OS);


SECTIONS {
    /* We use this for block device emulation */
    .disk_image ORIGIN(DDR4_SDRAM) :
    {
        _disk_start = .;
        KEEP(*(.disk_image));
        _disk_end = .;
    } > DDR4_SDRAM
} INSERT BEFORE .text;
