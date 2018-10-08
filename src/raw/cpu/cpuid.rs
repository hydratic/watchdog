#![no_std]

extern crate raw-cpuid;

// TODO:
// Rest
macro_rules! cpuid {
    ($arch:expr) => {{
        if $arch == x86 {
            let cpuid = CpuId::new();

            // Cache Parameters
            match cpuid.get_cache_parameters() {
                Some(cparams) => {
                    for cache in cparams {
                        let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
                        println!("L{}-Cache size is {}", cache.level(), size);
                    }
                },
                None => println!("No cache parameter information available"),
            }

            // SSE
            let has_sse = match cpuid.get_feature_info() {
                Some(finfo) => finfo.has_sse(),
                None => false
            };

            // Vendors
            let has_sse = match cpuid.get_vendor_info() {
                Some(vf) => assert!(vf.as_string() == "GenuineIntel"),
                None => ()			
            };
        }
    }};
}
