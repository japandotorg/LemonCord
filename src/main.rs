extern crate wry;
extern crate anyhow;

use anyhow::Result;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::{webview_version, WebViewBuilder},
};

pub const DISCORD: &str = "https://discord.com/app";
pub const APP_NAME: &str = "LemonCord";

fn main() -> Result<()> {
    let version_info = env!("CARGO_PKG_VERSION");

    let webview_version_info = webview_version().unwrap();

    // Most common User-Agent.
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_transparent(true)
        .with_resizable(true)
        .build(&event_loop)
        .unwrap_or_else(
            |_| 
                panic!(
                    "Unable to build window!"
                )
        );

    #[allow(unused_mut)]
    let mut _webview = WebViewBuilder::new(window)?
        .with_user_agent(&user_agent)
        .with_devtools(cfg!(any(debug_assertions, feature = "devtools")))
        .with_url(DISCORD)?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Discord web view successfully started."),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
