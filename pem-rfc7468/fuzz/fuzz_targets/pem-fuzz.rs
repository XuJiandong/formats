#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut buf = vec![0u8; data.len()*2 + 1024];
    let _ = pem_rfc7468::decode(data, &mut buf);
});
