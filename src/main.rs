extern crate native_windows_gui;
extern crate once_cell;
extern crate webview2;
extern crate winapi;


use native_windows_gui::{
    self as nwg, 
    Window
};
use once_cell::unsync::OnceCell;
use winapi::um::winuser::*;
use webview2::Controller;
use std::rc::Rc;
use std::mem;


pub const DISCORD: &str = "https://discord.com/app";
pub const APP_NAME: &str = "LemonCord";

fn load_icon() -> nwg::Icon {
    nwg::Icon::from_file("assets/logo.ico", true).unwrap()
}


fn main() {
    #[allow(deprecated)]
    unsafe {
        nwg::set_dpi_awareness()
    }

    nwg::init().unwrap();

    let mut window = Window::default();

    Window::builder()
        .title(&APP_NAME.to_string())
        .size((1600, 900))
        .icon(Some(&load_icon()))
        .build(&mut window)
        .unwrap();

    let window_handle = window.handle;

    let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());
    let controller_clone: Rc<OnceCell<Controller>> = controller.clone();

    let result = webview2::Environment::builder().build(move |env| {
        env.unwrap()
            .create_controller(window_handle.hwnd().unwrap(), move |c| {
                let c = c.unwrap();

                unsafe {
                    let mut rect = mem::zeroed();
                    GetClientRect(window_handle.hwnd().unwrap(), &mut rect);
                    c.put_bounds(rect).unwrap();
                }

                let webview = c.get_webview().unwrap();
                webview.navigate(&DISCORD.to_string()).unwrap();

                controller_clone.set(c).unwrap();
                Ok(())
            })
    });
    if let Err(e) = result {
        nwg::modal_fatal_message(
            &window_handle,
            "Failed to Create The WebView Environment",
            &format!("{}", e),
        );
    }

    let window_handle = window.handle;

    nwg::bind_raw_event_handler(&window_handle, 0xffff + 1, move |_, msg, w, _| {
        match (msg, w as usize) {
            (WM_SIZE, _) => {
                if let Some(controller) = controller.get() {
                    unsafe {
                        let mut rect = mem::zeroed();
                        GetClientRect(window_handle.hwnd().unwrap(), &mut rect);
                        controller.put_bounds(rect).unwrap();
                    }
                }
            }
            (WM_MOVE, _) => {
                if let Some(controller) = controller.get() {
                    controller.notify_parent_window_position_changed().unwrap();
                }
            }
            (WM_SYSCOMMAND, SC_MINIMIZE) => {
                if let Some(controller) = controller.get() {
                    controller.put_is_visible(false).unwrap();
                }
            }
            (WM_SYSCOMMAND, SC_RESTORE) => {
                if let Some(controller) = controller.get() {
                    controller.put_is_visible(true).unwrap();
                }
            }
            (WM_CLOSE, _) => nwg::stop_thread_dispatch(),
            _ => {}
        }
        None
    })
    .unwrap();

    nwg::dispatch_thread_events();
}
