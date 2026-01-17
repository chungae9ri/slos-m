/* Memory layout for LPC55S69 Core 0 */
MEMORY
{
  /* The LPC55S69 has 640KB Flash starting at 0x0 */
  FLASH : ORIGIN = 0x00000000, LENGTH = 640K
  
  /* RAM is split into multiple blocks; 0x20000000 is the main SRAM */
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

/* Define the start of the stack at the end of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);