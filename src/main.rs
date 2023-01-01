extern crate wry;

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, self},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

const DISCORD: &str = "https://discord.com/app";

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("LemonCord")
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
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
