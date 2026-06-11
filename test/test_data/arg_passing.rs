// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern "C" {
    fn proxy_log(level: u32, message_data: *const u8, message_size: usize) -> bool;
}

fn log(message: &str) {
    unsafe {
        proxy_log(/*error*/ 4, message.as_bytes().as_ptr(), message.len());
    }
}

#[no_mangle]
pub extern "C" fn _initialize() {
    std::panic::set_hook(Box::new(|panic_info| {
        log(&format!(
            "panic message: {}",
            panic_info.payload_as_str().unwrap_or("")
        ));
    }));
}

#[no_mangle]
pub extern "C" fn proxy_abi_version_0_2_0() {}

#[no_mangle]
pub extern "C" fn proxy_on_memory_allocate(size: usize) -> *mut u8 {
    let mut vec: Vec<u8> = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
    let slice = vec.into_boxed_slice();
    Box::into_raw(slice) as *mut u8
}

#[no_mangle]
pub extern "C" fn test_pass_u32(int: u32) -> u32 {
    return int;
}

#[no_mangle]
pub extern "C" fn test_pass_u64(int: u64) -> u64 {
    return int;
}
