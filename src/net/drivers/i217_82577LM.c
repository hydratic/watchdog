class MMIOUtils
{
    public:
        static uint8_t read8 (uint64_t p_address);
        static uint16_t read16 (uint64_t p_address);
        static uint32_t read32 (uint64_t p_address);
        static uint64_t read64 (uint64_t p_address);
        static void write8 (uint64_t p_address,uint8_t p_value);
        static void write16 (uint64_t p_address,uint16_t p_value);
        static void write32 (uint64_t p_address,uint32_t p_value);
        static void write64 (uint64_t p_address,uint64_t p_value);
};
 
#endif /* MMIOUTILS_H_ */
 
 
 
uint8_t MMIOUtils::read8 (uint64_t p_address)
{
    return *((volatile uint8_t*)(p_address));
}
uint16_t MMIOUtils::read16 (uint64_t p_address)
{
    return *((volatile uint16_t*)(p_address));
 
}
uint32_t MMIOUtils::read32 (uint64_t p_address)
{
    return *((volatile uint32_t*)(p_address));
 
}
uint64_t MMIOUtils::read64 (uint64_t p_address)
{
    return *((volatile uint64_t*)(p_address));    
}
void MMIOUtils::write8 (uint64_t p_address,uint8_t p_value)
{
    (*((volatile uint8_t*)(p_address)))=(p_value);
}
void MMIOUtils::write16 (uint64_t p_address,uint16_t p_value)
{
    (*((volatile uint16_t*)(p_address)))=(p_value);    
}
void MMIOUtils::write32 (uint64_t p_address,uint32_t p_value)
{
    (*((volatile uint32_t*)(p_address)))=(p_value);
 
}
void MMIOUtils::write64 (uint64_t p_address,uint64_t p_value)
{
    (*((volatile uint64_t*)(p_address)))=(p_value);    
}
#ifndef PORTS_H_
#define PORTS_H_
 
 
class Ports
{
    private:
    public:
        static void outportb (uint16_t p_port,uint8_t data);
        static void outportw (uint16_t p_port,uint16_t data);
        static void outportl (uint16_t p_port,uint32_t data);
        static uint8_t inportb( uint16_t p_port);
        static uint16_t inportw( uint16_t p_port);
        static uint32_t inportl( uint16_t p_port);
};
 
#endif /* PORTS_H_ */
 
 
/* void Ports::outportb (uint16_t p_port,uint8_t p_data)
 * 
 * This method outputs a byte to a hardware port.
 * It uses an inline asm with the volatile keyword
 * to disable compiler optimization.
 * 
 *  p_port: the port number to output the byte p_data to.
 *  p_data: the byte to to output to the port p_port.
 * 
 * Notice the input constraint
 *      "dN" (port) : indicates using the DX register to store the 
 *                  value of port in it
 *      "a"  (data) : store the value of data into 
 * 
 * The above constraint will instruct the compiler to generate assembly
 * code that looks like that
 *      mov    %edi,%edx
 *      mov    %esi,%eax
 *      out    %eax,(%dx)
 * 
 * According the ABI, the edi will have the value of p_port and esi will have
 * the value of the p_data
 * 
 */
void Ports::outportb (uint16_t p_port,uint8_t p_data)
{
    asm volatile ("outb %1, %0" : : "dN" (p_port), "a" (p_data));
}
 
/* void Ports::outportw (uint16_t p_port,uint16_t p_data)
 * 
 * This method outputs a word to a hardware port.
 * 
 *  p_port: the port number to output the byte p_data to.
 *  p_data: the byte to to output to the port p_port.
 * 
 */
 
 
void Ports::outportw (uint16_t p_port,uint16_t p_data)
{
    asm volatile ("outw %1, %0" : : "dN" (p_port), "a" (p_data));
}
 
/* void Ports::outportl (uint16_t p_port,uint32_t p_data)
 * 
 * This method outputs a double word to a hardware port.
 * 
 *  p_port: the port number to output the byte p_data to.
 *  p_data: the byte to to output to the port p_port.
 * 
 */
 
 
void Ports::outportl (uint16_t p_port,uint32_t p_data)
{
    asm volatile ("outl %1, %0" : : "dN" (p_port), "a" (p_data));
}
 
/* uint8_t Ports::inportb( uint16_t p_port)
 * 
 * This method reads a byte from a hardware port.
 * 
 *  p_port: the port number to read the byte from.
 *  return value : a byte read from the port p_port.
 * 
 * Notice the output constraint "=a", this tells the compiler 
 * to expect the save the value of register AX into the variable l_ret
 * The register AX should contain the result of the inb instruction.
 * 
 * 
 */
 
uint8_t Ports::inportb( uint16_t p_port)
{
    uint8_t l_ret;
    asm volatile("inb %1, %0" : "=a" (l_ret) : "dN" (p_port));
    return l_ret;
}
 
/* uint16_t Ports::inportw( uint16_t p_port)
 * 
 * This method reads a word from a hardware port.
 * 
 *  p_port: the port number to read the word from.
 *  return value : a word read from the port p_port.
 * 
 */
 
 
uint16_t Ports::inportw( uint16_t p_port)
{
    uint16_t l_ret;
    asm volatile ("inw %1, %0" : "=a" (l_ret) : "dN" (p_port));
    return l_ret;
}
 
 
/* uint16_t Ports::inportl( uint16_t p_port)
 * 
 * This method reads a double word from a hardware port.
 * 
 *  p_port: the port number to read the double word from.
 *  return value : a double word read from the port p_port.
 * 
 */
 
uint32_t Ports::inportl( uint16_t p_port)
{
    uint32_t l_ret;
    asm volatile ("inl %1, %0" : "=a" (l_ret) : "dN" (p_port));
    return l_ret;
}

class E1000 : public NetworkDriver
{
    private:
 
        uint8_t bar_type;     // Type of BOR0
        uint16_t io_base;     // IO Base Address
        uint64_t  mem_base;   // MMIO Base Address
        bool eerprom_exists;  // A flag indicating if eeprom exists
        uint8_t mac [6];      // A buffer for storing the mack address
        struct e1000_rx_desc *rx_descs[E1000_NUM_RX_DESC]; // Receive Descriptor Buffers
        struct e1000_tx_desc *tx_descs[E1000_NUM_TX_DESC]; // Transmit Descriptor Buffers
        uint16_t rx_cur;      // Current Receive Descriptor Buffer
        uint16_t tx_cur;      // Current Transmit Descriptor Buffer
 
 
        // Send Commands and read results From NICs either using MMIO or IO Ports
        void writeCommand( uint16_t p_address, uint32_t p_value);
        uint32_t readCommand(uint16_t p_address);
 
 
        bool detectEEProm(); // Return true if EEProm exist, else it returns false and set the eerprom_existsdata member
        uint32_t eepromRead( uint8_t addr); // Read 4 bytes from a specific EEProm Address
        bool readMACAddress();       // Read MAC Address
        void startLink ();           // Start up the network
        void rxinit();               // Initialize receive descriptors an buffers
        void txinit();               // Initialize transmit descriptors an buffers
        void enableInterrupt();      // Enable Interrupts
        void handleReceive();        // Handle a packet reception.
    public:
 
        E1000(PCIConfigHeader * _pciConfigHeader); // Constructor. takes as a parameter a pointer to an object that encapsulate all he PCI configuration data of the device
        void start ();                             // perform initialization tasks and starts the driver
        void fire (InterruptContext * p_interruptContext);  // This method should be called by the interrupt handler 
        uint8_t * getMacAddress ();                         // Returns the MAC address
        int sendPacket(const void * p_data, uint16_t p_len);  // Send a packet
        ~E1000();                                             // Default Destructor
};
