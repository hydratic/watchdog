#[warn(unused_imports)]
#![no_std]

#[macro_use]
mod vga;
mod fs;
mod memory;

const ELFIdent: i16 = 16;

#[packed]
struct ELFHeader {
    e_ident: ELFIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

struct ProgramHeader {
    p_type: u32,
    p_flags64: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags32: u32,
    p_align: u32,
}

struct SectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}


unsafe fn jmp(addr: u32) {
    asm!("jmp *($0)"
         :
         : "r" (addr));
}

// Executes a file starting at `addr`
pub unsafe fn exec(addr: uint) {
    let bytes: &[u8] = transmute(Slice {data: (addr as *u8), len: 100});
    let header = elf::read_header(bytes);
    assert(header.e_ident.ei_mag.slice(1,4) == "ELF");
    // Read the program header and load the program into memory at
    // the right address
    jmp(header.e_entry);
}
