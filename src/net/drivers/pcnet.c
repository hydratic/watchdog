/* pcnet.c
 * A driver for AMD PCNET networking
 *
 */

/* SWSTYLE:
 * uint32_t csr58 = readCSR32(58);
 * csr58 &= 0xfff0;
 * csr58 |= 2;
 * writeCSR32(58, csr58);
 *
 * ASEL:
 * uint32_t bcr2 = readBCR32(2);
 * bcr2 |= 0x2;
 * writeBCR32(2, bcr2);
 */
 
/* INIT
 * uint32_t conf = pciConfigReadDWord(bus, slot, func, offset);
 * conf &= 0xffff0000; // preserve status register, clear config register
 * conf |= 0x5;        // set bits 0 and 2
 * pciConfigWriteDWord(bus, slot, func, offset, conf);
 */
 
void writeRAP32(uint32_t val)
{
    outd(io_base + 0x14, val);
}
 
void writeRAP16(uint16_t val)
{
    outw(io_base + 0x12, val);
}
 
uint32_t readCSR32(uint32_t csr_no)
{
    writeRAP32(csr_no);
    return ind(io_base + 0x10);
}
 
uint16_t readCSR16(uint16_t csr_no)
{
    writeRAP32(csr_no);
    return inw(io_base + 0x10);
}
 
void writeCSR32(uint32_t csr_no, uint32_t val)
{
    writeRAP32(csr_no);
    outd(io_base + 0x10, val);
}
 
void writeCSR16(uint16_t csr_no, uint16_t val)
{
    writeRAP16(csr_no);
    outw(io_base + 0x10, val);
}

int rx_buffer_ptr = 0;
int tx_buffer_ptr = 0;                 // pointers to transmit/receive buffers
 
int rx_buffer_count = 32;              // total number of receive buffers
int tx_buffer_count = 8;               // total number of transmit buffers
 
const int buffer_size = 1548;          // length of each packet buffer
 
const int de_size = 16;                // length of descriptor entry
 
uint8_t *rdes;                         // pointer to ring buffer of receive DEs
uint8_t *tdes;                         // pointer to ring buffer of transmit DEs
 
uint32_t rx_buffers;                   // physical address of actual receive buffers (< 4 GiB)
uint32_t tx_buffers;                   // physical address of actual transmit buffers (< 4 GiB)
 
// does the driver own the particular buffer?
int driverOwns(uint8_t *des, int idx)
{
    return (des[de_size * idx + 7] & 0x80) == 0;
}
 
// get the next transmit buffer index
int nextTxIdx(int cur_tx_idx)
{
    int ret = cur_tx_idx + 1;
    if(ret == tx_buffer_count)
        ret = 0;
    return ret;
}
 
// get the next receive buffer index
int nextRxIdx(int cur_rx_idx)
{
    int ret = cur_rx_idx + 1;
    if(ret == rx_buffer_count)
        ret = 0;
    return ret;
}
 
// initialize a DE
void initDE(uint8_t *des, int idx, int is_tx)
{
    memset(&des[idx * de_size], de_size, 0);
 
    // first 4 bytes are the physical address of the actual buffer
    uint32_t buf_addr = rx_buffers;
    if(is_tx)
        buf_addr = tx_buffers;
    *(uint32_t *)&des[idx * de_size] = buf_addr + idx * buffer_size;
 
    // next 2 bytes are 0xf000 OR'd with the first 12 bits of the 2s complement of the length
    uint16_t bcnt = (uint16_t)(-buffer_size);
    bcnt &= 0x0fff;
    bcnt |= 0xf000;
    *(uint16_t *)&des[idx * de_size + 4] = bcnt;
 
    // finally, set ownership bit - transmit buffers are owned by us, receive buffers by the card
    if(!is_tx)
        des[idx * de_size + 7] = 0x80;
}

int sendPacket(void *packet, size_t len, uint8_t *dest)
{
    // the next available descriptor entry index is in tx_buffer_ptr
    if(!driverOwns(tdes, tx_buffer_ptr))
    {
        // we don't own the next buffer, this implies all the transmit
        //  buffers are full and the card hasn't sent them yet.
        // A fully functional driver would therefore add the packet to
        //  a queue somewhere, and wait for the transmit done interrupt
        //  then try again.  We simply fail and return.  You can set
        //  bit 3 of CSR0 here to encourage the card to send all buffers.
        return 0;
    }
 
    // copy the packet data to the transmit buffer.  An alternative would
    //  be to update the appropriate transmit DE to point to 'packet', but
    //  then you would need to ensure that packet is not invalidated before
    //  the card has a chance to send the data.
    memcpy((void *)(tx_buffers + tx_buffer_ptr * buffer_size), packet, len);
 
    // set the STP bit in the descriptor entry (signals this is the first
    //  frame in a split packet - we only support single frames)
    tdes[tx_buffer_ptr * de_size + 7] |= 0x2;
 
    // similarly, set the ENP bit to state this is also the end of a packet
    tdes[tx_buffer_ptr * de_size + 7] |= 0x1;
 
    // set the BCNT member to be 0xf000 OR'd with the first 12 bits of the
    //  two's complement of the length of the packet
    uint16_t bcnt = (uint16_t)(-len);
    bcnt &= 0xfff;
    bcnt |= 0xf000;
    *(uint16_t *)&tdes[tx_buffer_ptr * de_size + 4] = bcnt;
 
    // finally, flip the ownership bit back to the card
    tdes[tx_buffer_ptr * de_size + 7] |= 0x80;
 
    // update the next transmit pointer
    tx_buffer_ptr = nextTxIdx(tx_buffer_ptr);
}

void handleReceiveInterrupt()
{
    while(driverOwns(rdes, rx_buffer_ptr))
    {
        // packet length is given by bytes 8 and 9 of the descriptor
        //  (no need to negate it unlike BCNT above)
        uint16_t plen = *(uint16_t *)&rdes[rx_buffer_ptr * de_size + 8];
 
        // the packet itself is written somewhere in the receive buffer
        void *pbuf = (void *)(rx_buffers + rx_buffer_ptr * buffer_size);
 
        // do something with the packet (i.e. hand to the next layer in the
        //  network stack).  You probably don't want to do any extensive
        //  processing here (as this is within an interrupt handler) - just
        //  copy the data somewhere to a queue and continue so that the
        //  system is interrupted for as little time as possible
        handlePacket(pbuf, plen);
 
        // hand the buffer back to the card
        rdes[rx_buffer_ptr * de_size + 7] = 0x80;
 
        // increment rx_buffer_ptr;
        rx_buffer_ptr = nextRxIdx(rx_buffer_ptr);
    }
 
    // set interrupt as handled
    writeCSR32(readCSR32(0) | 0x0400);
 
    // don't forget to send EOI
}
