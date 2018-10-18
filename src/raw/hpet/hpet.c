struct address_structure
{
    uint8_t address_space_id;    // 0 - system memory, 1 - system I/O
    uint8_t register_bit_width;
    uint8_t register_bit_offset;
    uint8_t reserved;
    uint64_t address;
} __attribute__((packed));
 
struct description_table_header
{
    char signature[4];    // 'HPET' in case of HPET table
    uint32_t length;
    uint8_t revision;
    uint8_t checksum;
    char oemid[6];
    uint64_t oem_tableid;
    uint32_t oem_revision;
    uint32_t creator_id;
    uint32_t creator_revision;
} __attribute__((packed));
 
struct hpet : public description_table_header
{
    uint8_t hardware_rev_id;
    uint8_t comparator_count:5;
    uint8_t counter_size:1;
    uint8_t reserved:1;
    uint8_t legacy_replacement:1;
    uint16_t pci_vendor_id;
    address_structure address;
    uint8_t hpet_number;
    uint16_t minimum_tick;
    uint8_t page_protection;
} __attribute__((packed));

int one_shot_mode() {
  if (time < COUNTER_CLK_PERIOD)
  {
      time = adjust_time(time);
  }
 
  write_register_64(timer_configuration(n), (ioapic_input << 9) | (1 << 2));
  write_register_64(timer_comparator(n), read_register(main_counter) + time);
}

// "time" is time in femtoseconds from now to interrupt
int periodic_mode() {
  if (time < COUNTER_CLK_PERIOD)
  {
      time = adjust_time(time);
  }
 
  write_register_64(timer_configuration(n), (ioapic_input << 9) | (1 << 2) | (1 << 3) | (1 << 6));
  write_register_64(timer_comparator(n), read_register(main_counter) + time);
  write_register_64(timer_comparator(n), time);
}
