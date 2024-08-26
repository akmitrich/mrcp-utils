use std::collections::HashMap;

use crate::{inline_mrcp_resource_header_get, inline_mrcp_resource_header_property_check, uni};

#[derive(Debug)]
pub struct RecogHeaders {
    sensitivity: Option<f64>,
    noinput_timeout: Option<usize>,
    recognition_timeout: Option<usize>,
    start_input_timers: Option<bool>,
    silence_timeout: Option<usize>,
    pub vendor_specific: HashMap<String, String>,
}

impl RecogHeaders {
    pub fn new(request: *const uni::mrcp_message_t) -> Self {
        Self {
            sensitivity: extract_sensitivity(request),
            noinput_timeout: extract_noinput_timeout(request),
            recognition_timeout: extract_recognition_timeout(request),
            start_input_timers: extract_start_input_timers(request),
            silence_timeout: extract_speech_complete_timeout(request),
            vendor_specific: super::extract_vendor_specific_parameters(request),
        }
    }

    pub fn sensitivity(&self) -> f64 {
        self.sensitivity.unwrap_or(0.6)
    }

    pub fn noinput_timeout(&self) -> usize {
        self.noinput_timeout.unwrap_or(5000)
    }

    pub fn recognition_timeout(&self) -> usize {
        self.recognition_timeout.unwrap_or(20000)
    }

    pub fn start_input_timers(&self) -> bool {
        self.start_input_timers.unwrap_or(true)
    }

    pub fn silence_timeout(&self) -> usize {
        self.silence_timeout.unwrap_or(1000)
    }
}

fn extract_sensitivity(request: *const uni::mrcp_message_t) -> Option<f64> {
    if request.is_null() {
        return None;
    }
    unsafe {
        if inline_mrcp_resource_header_property_check(
            request,
            uni::RECOGNIZER_HEADER_SENSITIVITY_LEVEL as _,
        ) == uni::TRUE
        {
            let recog_header =
                inline_mrcp_resource_header_get(request) as *mut uni::mrcp_recog_header_t;
            if recog_header.is_null() {
                None
            } else {
                Some((*recog_header).sensitivity_level as _)
            }
        } else {
            None
        }
    }
}

fn extract_noinput_timeout(request: *const uni::mrcp_message_t) -> Option<usize> {
    if request.is_null() {
        return None;
    }
    unsafe {
        if inline_mrcp_resource_header_property_check(
            request,
            uni::RECOGNIZER_HEADER_NO_INPUT_TIMEOUT as _,
        ) == uni::TRUE
        {
            let recog_header =
                inline_mrcp_resource_header_get(request) as *mut uni::mrcp_recog_header_t;
            if recog_header.is_null() {
                None
            } else {
                Some((*recog_header).no_input_timeout)
            }
        } else {
            None
        }
    }
}

fn extract_recognition_timeout(request: *const uni::mrcp_message_t) -> Option<usize> {
    if request.is_null() {
        return None;
    }
    unsafe {
        if inline_mrcp_resource_header_property_check(
            request,
            uni::RECOGNIZER_HEADER_RECOGNITION_TIMEOUT as _,
        ) == uni::TRUE
        {
            let recog_header =
                inline_mrcp_resource_header_get(request) as *mut uni::mrcp_recog_header_t;
            if recog_header.is_null() {
                None
            } else {
                Some((*recog_header).recognition_timeout)
            }
        } else {
            None
        }
    }
}

fn extract_start_input_timers(request: *const uni::mrcp_message_t) -> Option<bool> {
    if request.is_null() {
        return None;
    }
    unsafe {
        if inline_mrcp_resource_header_property_check(
            request,
            uni::RECOGNIZER_HEADER_START_INPUT_TIMERS as _,
        ) == uni::TRUE
        {
            let recog_header =
                inline_mrcp_resource_header_get(request) as *mut uni::mrcp_recog_header_t;
            if recog_header.is_null() {
                None
            } else {
                Some((*recog_header).start_input_timers == uni::TRUE)
            }
        } else {
            None
        }
    }
}

fn extract_speech_complete_timeout(request: *const uni::mrcp_message_t) -> Option<usize> {
    if request.is_null() {
        return None;
    }
    unsafe {
        if inline_mrcp_resource_header_property_check(
            request,
            uni::RECOGNIZER_HEADER_SPEECH_COMPLETE_TIMEOUT as _,
        ) == uni::TRUE
        {
            let recog_header =
                inline_mrcp_resource_header_get(request) as *mut uni::mrcp_recog_header_t;
            if recog_header.is_null() {
                None
            } else {
                match (*recog_header).speech_complete_timeout {
                    0..=1 => Some(1000),
                    timeout @ 2..=4 => Some(timeout * 1000),
                    5..=20 => Some(1200),
                    value => Some(value),
                }
            }
        } else {
            None
        }
    }
}
