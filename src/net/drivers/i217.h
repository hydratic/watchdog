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
