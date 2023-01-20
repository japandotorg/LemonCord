extern crate wry;
extern crate image;
extern crate anyhow;

use anyhow::Result;

use wry::{
    application::{
        platform::windows::WindowBuilderExtWindows,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, Icon}
    },
    webview::{webview_version, WebViewBuilder},
};


pub const DISCORD: &str = "https://discord.com/app";
pub const APP_NAME: &str = "LemonCord";


fn main() -> Result<()> {
    #[allow(unused_mut)]
    let mut icon = image::open("assets/logo.webp")
        .expect("Failed to open icon path.") 
        .to_rgba8();

    let (icon_width, icon_height) = icon.dimensions();

    let version_info = env!("CARGO_PKG_VERSION");

    let webview_version_info = webview_version().unwrap();

    let user_agent = format!(
        "{} v{} ({}; {})",
        APP_NAME,
        version_info,
        std::env::consts::OS,
        webview_version_info
    );

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_window_icon(Some(
            Icon::from_rgba(icon.clone().into_raw(), icon_width, icon_height).unwrap(),
        )) 
        .with_taskbar_icon(Some(
            Icon::from_rgba(icon.clone().into_raw(), icon_width, icon_height).unwrap(),
        ))
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
