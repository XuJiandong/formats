#![no_main]

use libfuzzer_sys::fuzz_target;
use pkcs8::PrivateKeyInfo;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let _ = PrivateKeyInfo::try_from(data);
});
