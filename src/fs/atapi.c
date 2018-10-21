/* The default and seemingly universal sector size for CD-ROMs. */
#define ATAPI_SECTOR_SIZE 2048
 
/* The default ISA IRQ numbers of the ATA controllers. */
#define ATA_IRQ_PRIMARY     0x0E
#define ATA_IRQ_SECONDARY   0x0F
 
/* The necessary I/O ports, indexed by "bus". */
#define ATA_DATA(x)         (x)
#define ATA_FEATURES(x)     (x+1)
#define ATA_SECTOR_COUNT(x) (x+2)
#define ATA_ADDRESS1(x)     (x+3)
#define ATA_ADDRESS2(x)     (x+4)
#define ATA_ADDRESS3(x)     (x+5)
#define ATA_DRIVE_SELECT(x) (x+6)
#define ATA_COMMAND(x)      (x+7)
#define ATA_DCR(x)          (x+0x206)   /* device control register */
 
/* valid values for "bus" */
#define ATA_BUS_PRIMARY     0x1F0
#define ATA_BUS_SECONDARY   0x170
/* valid values for "drive" */
#define ATA_DRIVE_MASTER    0xA0
#define ATA_DRIVE_SLAVE     0xB0
 
/* ATA specifies a 400ns delay after drive switching -- often
 * implemented as 4 Alternative Status queries. */
#define ATA_SELECT_DELAY(bus) \
  {inb(ATA_DCR(bus));inb(ATA_DCR(bus));inb(ATA_DCR(bus));inb(ATA_DCR(bus));}
 
/* Use the ATAPI protocol to read a single sector from the given
 * bus/drive into the buffer using logical block address lba. */
int
atapi_drive_read_sector (uint32_t bus, uint32_t drive, uint32_t lba, uint8_t *buffer)
{
	/* 0xA8 is READ SECTORS command byte. */
	uint8_t read_cmd[12] = { 0xA8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
	uint8_t status;
	int size;
	/* Tell the scheduler that this process is using the ATA subsystem. */
	ata_grab ();
	/* Select drive (only the slave-bit is set) */
	outb (drive & (1 << 4), ATA_DRIVE_SELECT (bus));      
	ATA_SELECT_DELAY (bus);       /* 400ns delay */
	outb (0x0, ATA_FEATURES (bus));       /* PIO mode */
	outb (ATAPI_SECTOR_SIZE & 0xFF, ATA_ADDRESS2 (bus));
	outb (ATAPI_SECTOR_SIZE >> 8, ATA_ADDRESS3 (bus));
	outb (0xA0, ATA_COMMAND (bus));       /* ATA PACKET command */
	while ((status = inb (ATA_COMMAND (bus))) & 0x80)     /* BUSY */
	asm volatile ("pause");
	while (!((status = inb (ATA_COMMAND (bus))) & 0x8) && !(status & 0x1))
	asm volatile ("pause");
	/* DRQ or ERROR set */
	if (status & 0x1) {
	size = -1;
	goto cleanup;
	}
	read_cmd[9] = 1;              /* 1 sector */
	read_cmd[2] = (lba >> 0x18) & 0xFF;   /* most sig. byte of LBA */
	read_cmd[3] = (lba >> 0x10) & 0xFF;
	read_cmd[4] = (lba >> 0x08) & 0xFF;
	read_cmd[5] = (lba >> 0x00) & 0xFF;   /* least sig. byte of LBA */
	/* Send ATAPI/SCSI command */
	outsw (ATA_DATA (bus), (uint16_t *) read_cmd, 6);
	/* Wait for IRQ that says the data is ready. */
	schedule ();
	/* Read actual size */
	size =
	(((int) inb (ATA_ADDRESS3 (bus))) << 8) |
	(int) (inb (ATA_ADDRESS2 (bus)));
	/* This example code only supports the case where the data transfer
	* of one sector is done in one step. */
	ASSERT (size == ATAPI_SECTOR_SIZE);
	/* Read data. */
	insw (ATA_DATA (bus), buffer, size / 2);
	/* The controller will send another IRQ even though we've read all
	* the data we want.  Wait for it -- so it doesn't interfere with
	* subsequent operations: */
	schedule ();
	/* Wait for BSY and DRQ to clear, indicating Command Finished */
	while ((status = inb (ATA_COMMAND (bus))) & 0x88) 
	asm volatile ("pause");
	cleanup:
	/* Exit the ATA subsystem */
	ata_release ();
	return size;
}
