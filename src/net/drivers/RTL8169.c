/* TODO: Add Result stuff so it can be used to define a variable */
int mac_addr()
 {
 	char i;
 	for (i = 0; i < 6; i++)
 	{
    	mac_address[i] = inportb(ioaddr + i); /*ioaddr is the base address obtainable from the PCI device configuration space.*/
 	}
}

 struct Descriptor
 {
     unsigned int command,  /* command/status uint32_t */
                  vlan,     /* currently unused */
                  low_buf,  /* low 32-bits of physical buffer address */
                  high_buf; /* high 32-bits of physical buffer address */
 };
 
 struct Descriptor *Rx_Descriptors = (struct Descriptor *)0x100000; /* 1MB Base Address of Rx Descriptors */
 struct Descriptor *Tx_Descriptors = (struct Descriptor *)0x200000; /* 2MB Base Address of Tx Descriptors *
 
 void setup_rx_descriptors()
 {
     /* rx_buffer_len is the size (in bytes) that is reserved for incoming packets */
     unsigned int OWN = 0x80000000, EOR = 0x40000000; /* Bit offsets */
     int i;
     for(i = 0; i < num_of_rx_descriptors; i++) /* num_of_rx_descriptors can be up to 1024 */
     {
         if(i == (num_of_rx_descriptors - 1)) /* Last descriptor? if so, set the EOR bit */
           Rx_Descriptors[i].command = (OWN | EOR | (rx_buffer_len & 0x3FFF));
         else
           Rx_Descriptors[i].command = (OWN | (rx_buffer_len & 0x3FFF));
         /* VLAN adjustments are not part of this guide at the moment - leave as zeros for normal operation */
         Rx_Descriptors[i].low_buf = (unsigned int)&packet_buffer_address; /* This is where the packet data will go */
         /* If you are programming for a 64-bit OS, put the high memory location in the 'high_buf' descriptor area */
     }
 }
 
 int init_driver()
 {
 	outportl(ioaddr + 0x44, 0x0000E70F) /* RXFTH: unlimited, MXDMA: unlimited, AAP: set (promisc. mode set) */
 	outportb(ioaddr + 0xEC, 0x3B);
	outportw(ioaddr + 0xDA, 0x1FFF); /* Maximum rx packet size */
 }
 
  void reset()
 {
     unsigned int i;
     unsigned char mac_address[6];
 
     outportb(ioaddr + 0x37, 0x10); /* Send the Reset bit to the Command register */
     while(inportb(ioaddr + 0x37) & 0x10){} /* Wait for the chip to finish resetting */
 
     for (i = 0; i < 6; i++)
       mac_address[i] = inportb(ioaddr + i);
 
     setup_rx_descriptors();
     outportb(ioaddr + 0x50, 0xC0); /* Unlock config registers */
     outportl(ioaddr + 0x44, 0x0000E70F); /* RxConfig = RXFTH: unlimited, MXDMA: unlimited, AAP: set (promisc. mode set) */
     outportb(ioaddr + 0x37, 0x04); /* Enable Tx in the Command register, required before setting TxConfig */
     outportl(ioaddr + 0x40, 0x03000700); /* TxConfig = IFG: normal, MXDMA: unlimited */
     outportw(ioaddr + 0xDA, 0x1FFF); /* Max rx packet size */
     outportb(ioaddr + 0xEC, 0x3B); /* max tx packet size */
 
     /* offset 0x20 == Transmit Descriptor Start Address Register 
        offset 0xE4 == Receive Descriptor Start Address Register 
 
        Again, these are *physical* addresses. This code assumes physical==linear, this is
        typically not the case in real world kernels
     */
     outportl(ioaddr + 0x20, (unsigned long)&Tx_Descriptors[0]; /* Tell the NIC where the first Tx descriptor is */
     outportl(ioaddr + 0xE4, (unsigned long)&Rx_Descriptors[0]; /* Tell the NIC where the first Rx descriptor is */
 
     outportb(ioaddr + 0x37, 0x0C); /* Enable Rx/Tx in the Command register */
     outportb(ioaddr + 0x50, 0x00); /* Lock config registers */
 }
