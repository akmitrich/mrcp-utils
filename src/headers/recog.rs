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

use crate::{inline_mrcp_resource_header_get, inline_mrcp_resource_header_property_check, uni};
use std::collections::HashMap;

#[derive(Debug)]
pub struct RecogHeaders {
    pub sensitivity: crate::Result<f64>,
    pub noinput_timeout: crate::Result<usize>,
    pub recognition_timeout: crate::Result<usize>,
    pub start_input_timers: crate::Result<bool>,
    pub silence_timeout: crate::Result<usize>,
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
        *self.sensitivity.as_ref().unwrap_or(&0.6)
    }

    pub fn noinput_timeout(&self) -> usize {
        *self.noinput_timeout.as_ref().unwrap_or(&5000)
    }

    pub fn recognition_timeout(&self) -> usize {
        *self.recognition_timeout.as_ref().unwrap_or(&20000)
    }

    pub fn start_input_timers(&self) -> bool {
        *self.start_input_timers.as_ref().unwrap_or(&true)
    }

    pub fn silence_timeout(&self) -> usize {
        *self.silence_timeout.as_ref().unwrap_or(&1000)
    }
}

fn extract_sensitivity(request: *const uni::mrcp_message_t) -> crate::Result<f64> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
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
                Err(crate::Error::NoSuchHeader(
                    uni::RECOGNIZER_HEADER_SENSITIVITY_LEVEL,
                ))
            } else {
                Ok((*recog_header).sensitivity_level as _)
            }
        } else {
            Err(crate::Error::NoSuchHeader(
                uni::RECOGNIZER_HEADER_SENSITIVITY_LEVEL,
            ))
        }
    }
}

fn extract_noinput_timeout(request: *const uni::mrcp_message_t) -> crate::Result<usize> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
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
                Err(crate::Error::NoSuchHeader(
                    uni::RECOGNIZER_HEADER_NO_INPUT_TIMEOUT,
                ))
            } else {
                Ok((*recog_header).no_input_timeout)
            }
        } else {
            Err(crate::Error::NoSuchHeader(
                uni::RECOGNIZER_HEADER_NO_INPUT_TIMEOUT,
            ))
        }
    }
}

fn extract_recognition_timeout(request: *const uni::mrcp_message_t) -> crate::Result<usize> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
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
                Err(crate::Error::NoSuchHeader(
                    uni::RECOGNIZER_HEADER_RECOGNITION_TIMEOUT,
                ))
            } else {
                Ok((*recog_header).recognition_timeout)
            }
        } else {
            Err(crate::Error::NoSuchHeader(
                uni::RECOGNIZER_HEADER_RECOGNITION_TIMEOUT,
            ))
        }
    }
}

fn extract_start_input_timers(request: *const uni::mrcp_message_t) -> crate::Result<bool> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
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
                Err(crate::Error::NoSuchHeader(
                    uni::RECOGNIZER_HEADER_START_INPUT_TIMERS,
                ))
            } else {
                Ok((*recog_header).start_input_timers == uni::TRUE)
            }
        } else {
            Err(crate::Error::NoSuchHeader(
                uni::RECOGNIZER_HEADER_START_INPUT_TIMERS,
            ))
        }
    }
}

fn extract_speech_complete_timeout(request: *const uni::mrcp_message_t) -> crate::Result<usize> {
    if request.is_null() {
        return Err(crate::Error::NullRequest);
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
                Err(crate::Error::NoSuchHeader(
                    uni::RECOGNIZER_HEADER_SPEECH_COMPLETE_TIMEOUT,
                ))
            } else {
                match (*recog_header).speech_complete_timeout {
                    0..=1 => Ok(1000),
                    timeout @ 2..=4 => Ok(timeout * 1000),
                    5..=20 => Ok(1200),
                    value => Ok(value),
                }
            }
        } else {
            Err(crate::Error::NoSuchHeader(
                uni::RECOGNIZER_HEADER_SPEECH_COMPLETE_TIMEOUT,
            ))
        }
    }
}
