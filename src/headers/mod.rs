// Copyright 2024 ООО Оптимумсити

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use crate::{inline_mrcp_generic_header_get, inline_mrcp_generic_header_property_check, uni};
use std::collections::HashMap;

mod synth;
pub use synth::SynthHeaders;

mod recog;
pub use recog::RecogHeaders;

fn extract_vendor_specific_parameters(
    request: *const uni::mrcp_message_t,
) -> HashMap<String, String> {
    if request.is_null() {
        return HashMap::new();
    }
    let mut params = HashMap::new();
    unsafe {
        if inline_mrcp_generic_header_property_check(
            request,
            uni::GENERIC_HEADER_VENDOR_SPECIFIC_PARAMS as _,
        ) == uni::TRUE
        {
            let generic_header = inline_mrcp_generic_header_get(request);
            if !generic_header.is_null() {
                let vendor_parameters = (*generic_header).vendor_specific_params;
                let pairs = (*vendor_parameters).elts as *mut uni::apt_pair_t;
                for offset in 0..(*vendor_parameters).nelts {
                    let pair = pairs.offset(offset as _) as *mut uni::apt_str_t;
                    let key = &*pair.offset(0);
                    let value = &*pair.offset(1);
                    if let (Ok(key), Ok(value)) = (apt_str_to_string(key), apt_str_to_string(value))
                    {
                        params.insert(key, value);
                    };
                }
            }
        }
    }
    params
}

fn apt_str_to_string(origin: &uni::apt_str_t) -> crate::Result<String> {
    unsafe {
        let ptr = origin.buf as *const u8;
        let len = origin.length;
        if len == 0 || ptr.is_null() {
            Ok(String::new())
        } else {
            let as_slice = std::slice::from_raw_parts(ptr, len);
            Ok(std::str::from_utf8(as_slice).map(ToOwned::to_owned)?)
        }
    }
}
