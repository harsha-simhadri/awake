//use std::ffi::errorcodes::*;
use futures::Future;
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
//     is_completed: bool,
// }

struct ReadValueFuture {
    file_handle: u32, // replace with actual type
    state: u32,
    really_done: bool, // replace with actual type
}

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

type Cb = extern "C" fn(*mut c_void);

// Callback function to be passed to the C++ side
pub extern "C" fn callback_function(context_handle: *mut c_void) {
    // Convert context handle pointer back to ContextHandle
    let context_handle: &mut ContextHandle =
        unsafe { &mut *(context_handle as *mut ContextHandle) };

    // Wake up the future using the context handle
    context_handle.wake_task();
}

// C++ function signature
// External function declaration to call the C++ function with callback
#[link(name = "c_ffi_async", kind = "static")]
extern "C" {
    fn rust_callback(result: *const c_char, error_code: i32);
    fn cpp_call_get_value(callback: Cb, context_handle: *mut c_void);
}

// Rust function to call the C++ function with callback
pub fn call_cpp_get_value(
    callback: extern "C" fn(*mut c_void),
    context_handle: &mut ContextHandle,
) {
    // Call the C++ function with the callback and context handle
    unsafe {
        cpp_call_get_value(callback, context_handle as *mut _ as *mut c_void);
    }
}

impl Future for ReadValueFuture {
    type Output = (); // replace with actual types

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("future polled");
        // check if the file is done reading
        if self.really_done {
            self.state = 25;
            println!("future ready done");
            Poll::Ready(())
        } else {
            let file_handle = self.file_handle;
            let mut context_handle = ContextHandle::default();
            // Set the waker in the context handle
            context_handle.set_waker(cx.waker().clone());

            call_cpp_get_value(callback_function, &mut context_handle);

            //update state to reading
            println!("future pending");
            self.really_done = true;
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi_async_test::*;

    #[tokio::test]
    async fn test_read_value_future() {
        let future = ReadValueFuture {
            file_handle: 0,
            state: 0,
            really_done: false,
        };

        tokio::spawn(async move {
            future.await;
        });
    }
}
