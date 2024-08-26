use std::collections::HashMap;

use crate::{
    inline_mrcp_generic_header_get, inline_mrcp_generic_header_property_check,
    inline_mrcp_resource_header_get, inline_mrcp_resource_header_property_check, uni,
};

use super::apt_str_to_string;

#[derive(Debug)]
pub struct SynthHeaders {
    content_length: Option<usize>,
    voice_name: Option<String>,
    body: Option<String>,
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
        self.content_length.unwrap_or_default()
    }

    pub fn voice_name(&self) -> &str {
        match &self.voice_name {
            Some(voice) => voice.as_str(),
            None => "",
        }
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

fn extract_content_length(request: *const uni::mrcp_message_t) -> Option<usize> {
    if request.is_null() {
        return None;
    }
    unsafe {
        if inline_mrcp_generic_header_property_check(
            request,
            uni::GENERIC_HEADER_CONTENT_LENGTH as _,
        ) == uni::TRUE
        {
            let generic_header = inline_mrcp_generic_header_get(request);
            if generic_header.is_null() {
                None
            } else {
                Some((*generic_header).content_length)
            }
        } else {
            None
        }
    }
}

fn extract_voice_name(request: *const uni::mrcp_message_t) -> Option<String> {
    if request.is_null() {
        return None;
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
                None
            } else {
                apt_str_to_string(&(*synth_header).voice_param.name)
            }
        } else {
            None
        }
    }
}

fn extract_body(request: *const uni::mrcp_message_t) -> Option<String> {
    if request.is_null() {
        return None;
    }
    unsafe { apt_str_to_string(&(*request).body) }
}
