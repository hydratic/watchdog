#![no_std]

extern crate ux;
extern crate volatile;

use volatile::ReadWrite;
use volatile::Volatile;

pub struct APIC {
	APICID: Volatile<u32>,
  	APICVER: Volatile<u32>,
	TASKPRIOR: Volatile<u32>,
	EOI: Volatile<u32>,
	APIC_APICID: Volatile<u32>,
	APIC_APICVER: Volatile<u32>,
	APIC_TASKPRIOR: Volatile<u32>,
	APIC_EOI: Volatile<u32>,
	APIC_LDR: Volatile<u32>,
	APIC_DFR: Volatile<u32>,
	APIC_SPURIOUS: Volatile<u32>,
	APIC_ESR: Volatile<u32>,
	APIC_ICRL: Volatile<u32>,
	APIC_ICRH: Volatile<u32>,
	APIC_LVT_TMR: Volatile<u32>
	APIC_LVT_PERF: Volatile<u32>,
	APIC_LVT_LINT0: Volatile<u32>,
	APIC_LVT_LINT1: Volatile<u32>,
	APIC_LVT_ERR: Volatile<u32>,
	APIC_TMRINITCNT: Volatile<u32>,
	APIC_TMRCURRCNT: Volatile<u32>,
	APIC_TMRDIV: Volatile<u32>,
	APIC_LAST: Volatile<u32>,
	APIC_DISABLE: Volatile<u32>,
	APIC_SW_ENABLE: Volatile<u32>,
	APIC_CPUFOCUS: Volatile<u32>,
	APIC_NMI: Volatile<u32>,
	TMR_PERIODIC: Volatile<u32>,
	TMR_BASEDIV: Volatile<u32>,
}

// TODO
pub fn sleep(ms: i64) {
	let mut apic = Apic {
			
	};
}
