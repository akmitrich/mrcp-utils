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

#![allow(clippy::not_unsafe_ptr_arg_deref)]
use crate::uni;
use std::sync::{atomic::AtomicUsize, Arc};

pub trait Shutdown {
    fn shutdown(self);
}

pub struct SafeEngine<E> {
    inner: Arc<E>,
    channel_counter: AtomicUsize,
}

impl<E> SafeEngine<E> {
    pub fn leaked(engine: E) -> *mut Self {
        Box::into_raw(Box::new(Self {
            inner: Arc::new(engine),
            channel_counter: AtomicUsize::new(0),
        }))
    }

    pub fn engine(&self) -> Arc<E> {
        Arc::clone(&self.inner)
    }

    pub fn channel_opened(&mut self) -> usize {
        1 + self
            .channel_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

impl<E: Shutdown> SafeEngine<E> {
    pub fn destroy(this: *mut Self) {
        if this.is_null() {
            return;
        }
        let this = unsafe { Box::from_raw(this) };
        if let Ok(engine_to_shutdown) = Arc::try_unwrap(this.inner) {
            engine_to_shutdown.shutdown();
        }
    }
}

pub fn get_attribute(engine: *const uni::mrcp_engine_t, key: &[u8]) -> Option<String> {
    unsafe {
        let name = key.as_ptr() as *const i8;
        let raw_value = uni::mrcp_engine_param_get(engine, name);
        if raw_value.is_null() {
            // log::error!("[TCS_ENGINE] Your TCS Engine have no parameter '{:?}'. Consider edit 'unimrcpserver.xml' conf file.", key);
            return None;
        }
        std::ffi::CStr::from_ptr(raw_value)
            .to_str()
            .inspect_err(|e| {
                // log::error!("Your secret key is non-UTF8. {:?}", e);
            })
            .map(ToOwned::to_owned)
            .ok()
    }
}
