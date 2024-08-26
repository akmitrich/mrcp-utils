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

use super::apt_str_to_string;
use crate::{
    inline_mrcp_generic_header_get, inline_mrcp_generic_header_property_check,
    inline_mrcp_resource_header_get, inline_mrcp_resource_header_property_check, uni,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SynthHeaders {
    pub content_length: crate::Result<usize>,
    pub voice_name: crate::Result<String>,
    pub body: crate::Result<String>,
    pub vendor_specific: HashMap<String, String>,
}

impl SynthHeaders {
    pub fn new(request: *const uni::mrcp_message_t) -> Self {
        Self {
            content_length: extract_content_length(request),
            voice_name: extract_voice_name(request),
            body: extract_body(request),
            vendor_specific: super::extract_vendor_specific_parameters(request),
        }
    }

    pub fn content_length(&self) -> usize {
        *self.content_length.as_ref().unwrap_or(&0)
    }

    pub fn voice_name(&self) -> &str {
        match &self.voice_name {
            Ok(voice) => voice.as_str(),
            _ => "",
        }
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref().ok()
    }
}

fn extract_content_length(request: *const uni::mrcp_message_t) -> crate::Result<usize> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
    }
    unsafe {
        if inline_mrcp_generic_header_property_check(
            request,
            uni::GENERIC_HEADER_CONTENT_LENGTH as _,
        ) == uni::TRUE
        {
            let generic_header = inline_mrcp_generic_header_get(request);
            if generic_header.is_null() {
                Err(crate::Error::NoSuchResourceHeader(
                    uni::GENERIC_HEADER_CONTENT_LENGTH,
                ))
            } else {
                Ok((*generic_header).content_length)
            }
        } else {
            Err(crate::Error::NoSuchResourceHeader(
                uni::GENERIC_HEADER_CONTENT_LENGTH,
            ))
        }
    }
}

fn extract_voice_name(request: *const uni::mrcp_message_t) -> crate::Result<String> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
    }
    unsafe {
        if inline_mrcp_resource_header_property_check(
            request,
            uni::SYNTHESIZER_HEADER_VOICE_NAME as _,
        ) == uni::TRUE
        {
            let synth_header =
                inline_mrcp_resource_header_get(request) as *mut uni::mrcp_synth_header_t;
            if synth_header.is_null() {
                Err(crate::Error::NoSuchResourceHeader(
                    uni::SYNTHESIZER_HEADER_VOICE_NAME,
                ))
            } else {
                apt_str_to_string(&(*synth_header).voice_param.name)
            }
        } else {
            Err(crate::Error::NoSuchResourceHeader(
                uni::SYNTHESIZER_HEADER_VOICE_NAME,
            ))
        }
    }
}

fn extract_body(request: *const uni::mrcp_message_t) -> crate::Result<String> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
    }
    unsafe { apt_str_to_string(&(*request).body) }
}
