//use std::ffi::errorcodes::*;
use futures::Future;
use std::default;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// use std::sync::{Arc, Mutex};
// use std::thread;
// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};
// use tokio::runtime::Handle;
// use tokio::task;

// // Define a global variable wrapped in a Mutex
// static GLOBAL_VALUE: Mutex<bool> = Mutex::new(false);

// fn update_taskcompletion_value(value: bool) {
//     let mut global_value = GLOBAL_VALUE.lock().unwrap();
//     let current_thread_id = thread::current().id();
//     println!(
//         "update_taskcompletion_value: Current thread ID: {:?}",
//         current_thread_id
//     );
//     *global_value = value;
// }

// // Define a function to read the global variable safely
// fn read_taskcompletion_value() -> bool {
//     let global_value = GLOBAL_VALUE.lock().unwrap();
//     let current_thread_id = thread::current().id();
//     println!(
//         "read_taskcompletion_value: Current thread ID: {:?}",
//         current_thread_id
//     );
//     *global_value
// }

// async fn read_file_context(file_path: &str, waker: Waker) -> io::Result<String> {
//     // Open the file
//     let mut file = File::open(file_path).await?;

//     // Read the content of the file into a buffer
//     let mut buffer = String::new();
//     println!("Rust::file reading rust async func");
//     file.read_to_string(&mut buffer).await?;
//     println!("awaking poll");
//     waker.wake();

//     Ok(buffer)
// }

// struct GetValueFuture {
// file_handle: u32, // replace with actual type
// state: u32,
//     is_completed: bool,
// }

#[derive(Default)]
pub struct ContextHandle {
    waker: Option<Waker>,
}

impl ContextHandle {
    // Function to set the waker
    pub fn set_waker(&mut self, waker: Waker) {
        self.waker = Some(waker);
    }

    // Function to wake up the future
    pub fn wake_task(&self) {
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }
}

// Callback function to be passed to the C++ side
pub extern "C" fn callback_function(context_handle: *mut c_void) {
    // Convert context handle pointer back to ContextHandle
    let context_handle: &mut ContextHandle =
        unsafe { &mut *(context_handle as *mut ContextHandle) };

    // Wake up the future using the context handle
    context_handle.wake_task();
}

type Cb = extern "C" fn(*mut c_void);

#[link(name = "c_ffi_async", kind = "static")]
extern "C" {
    fn rust_callback(result: *const c_char, error_code: i32);
    fn spin_and_call_back(callback: Cb, context_handle: *mut c_void);
    fn add_to_billion_and_call_back(sum: *mut u64, callback: Cb, context_handle: *mut c_void);
}

struct SpinFuture {
    is_done: bool, // replace with actual type
}

impl Future for SpinFuture {
    type Output = (); // replace with actual types

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_done {
            println!("future ready done");
            Poll::Ready(())
        } else {
            let mut context_handle = ContextHandle::default();
            context_handle.set_waker(cx.waker().clone());
            unsafe {
                spin_and_call_back(
                    callback_function,
                    &mut context_handle as *mut _ as *mut c_void,
                )
            };
            self.is_done = true;
            Poll::Pending
        }
    }
}

#[repr(C)]
struct AddToBillionFuture {
    sum: u64,
    is_done: bool,
}

impl default::Default for AddToBillionFuture {
    fn default() -> Self {
        AddToBillionFuture {
            sum: 0,
            is_done: false,
        }
    }
}

impl Future for AddToBillionFuture {
    type Output = u64;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_done {
            Poll::Ready(self.sum)
        } else {
            let mut context_handle = ContextHandle::default();
            context_handle.set_waker(cx.waker().clone());

            unsafe {
                add_to_billion_and_call_back(
                    &mut self.sum as *mut _ as *mut u64,
                    callback_function,
                    &mut context_handle as *mut _ as *mut c_void,
                )
            };
            self.is_done = true;
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi_async_test::*;

    #[tokio::test]
    async fn test_spin_future() {
        let future = SpinFuture { is_done: false };

        tokio::spawn(async move {
            future.await;
        });
    }

    #[tokio::test]
    async fn test_add_to_billion_future() {
        let future = AddToBillionFuture::default();
        let result = future.await;
        assert_eq!(result, (1_000_000_000u64/2)*999_999_999u64);
    }
}
