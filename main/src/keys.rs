use log::error;
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::ctypes::c_uint;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{RegisterHotKey, UnregisterHotKey};

pub type ListenerId = i32;

pub struct Listener {
    last_id: ListenerId,
    handlers: HashMap<ListenerId, Box<dyn Fn()>>,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn register_hook<CB: Fn() + 'static>(
        &mut self,
        modifiers: LPARAM,
        key: c_uint,
        handler: CB,
    ) -> Result<ListenerId, String> {
        self.last_id += 1;
        let id = self.last_id;
        let result = unsafe { RegisterHotKey(0 as HWND, id, modifiers.try_into().unwrap(), key) };
        if result == 0 {
            return Err("Failed to register hook".into());
        }

        self.handlers.insert(id, Box::new(handler));
        Ok(id)
    }

    pub fn listen(&self, running: Arc<AtomicBool>) {
        while running.load(Ordering::SeqCst) {
            unsafe {
                let mut msg = MaybeUninit::uninit().assume_init();
                while running.load(Ordering::SeqCst)
                    && winapi::um::winuser::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0
                {
                    if msg.wParam != 0 {
                        if let Some(handler) = self.handlers.get(&(msg.wParam as ListenerId)) {
                            handler();
                        }
                    }
                }
            }
        }
        println!("stopped");
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        for id in self.handlers.keys() {
            unsafe {
                let result = UnregisterHotKey(0 as HWND, *id);
                if result == 0 {
                    error!("Unable to unregister hotkey listener with id {}", id);
                }
            }
        }
    }
}
