#define CURRENT_YEAR        2017                            // Change this each year!
 
int century_register = 0x00;                                // Set by ACPI table parsing code if possible
 
unsigned char second;
unsigned char minute;
unsigned char hour;
unsigned char day;
unsigned char month;
unsigned int year;
 
void out_byte(int port, int value);
int in_byte(int port);
 
enum {
      cmos_address = 0x70,
      cmos_data    = 0x71
};
 
int get_update_in_progress_flag() {
      out_byte(cmos_address, 0x0A);
      return (in_byte(cmos_data) & 0x80);
}
 
unsigned char get_RTC_register(int reg) {
      out_byte(cmos_address, reg);
      return in_byte(cmos_data);
}
