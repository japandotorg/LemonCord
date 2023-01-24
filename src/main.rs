extern crate wry;
extern crate dirs;
extern crate image;
extern crate anyhow;


use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, Icon},
    },
    webview::WebViewBuilder,
};

#[cfg(target_os = "windows")]
use wry::application::platform::windows::WindowBuilderExtWindows;

#[cfg(target_os = "macos")]
use wry::application::platform::macos::WindowBuilderExtMacOS;

#[cfg(any(target_os = "linux", target_os = "windows"))]
use wry::webview::WebContext;


#[allow(dead_code)]
enum UserEvents {
    CloseWindow,
}


// Convenient type alias of ``anyhow::Result`` type with ``wry::Error`` for LemonCord.
pub type Result<T> = anyhow::Result<T, wry::Error>;


pub const DISCORD: &str = "https://discord.com/app";
pub const APP_NAME: &str = "LemonCord";


fn main() -> Result<()> {
    // Most common User-Agent.
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();

    let event_loop = EventLoop::<UserEvents>::with_user_event();

    let main_window = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_transparent(true)
        .with_resizable(true);

    #[cfg(target_os = "windows")]
    let window = {
        let mut icon_path = format!("assets/{}_logo.webp", APP_NAME);

        if !std::path::Path::new(&icon_path).exists() {
            icon_path = "assets/logo.webp".to_string();
        }

        let icon = load_icon(std::path::Path::new(&icon_path));

        main_window
            .with_window_icon(Some(icon.clone()))
            .with_taskbar_icon(Some(icon.clone()))
            .build(&event_loop)
            .unwrap_or_else(
                |_|
                    panic!(
                        "Unable to build window!"
                    )
            )
    };

    #[cfg(target_os = "linux")]
    let window = {
        let mut icon_path = format!("assets/{}_logo.webp", APP_NAME);

        if !std::path::Path::new(&icon_path).exists() {
            icon_path = "assets/logo.webp".to_string();
        }

        let icon = load_icon(std::path::Path::new(&icon_path));

        main_window
            .with_window_icon(Some(icon.clone()))
            .build(&event_loop)
            .unwrap_or_else(
                |_|
                    panic!(
                        "Unable to build window!"
                    )
            )
    };

    #[cfg(target_os = "macos")]
    let window = {
        main_window
            .with_fullsize_content_view(true)
            .with_titlebar_buttons_hidden(false)
            .with_titlebar_transparent(true)
            .with_title_hidden(false)
            .build(&event_loop)
            .unwrap_or_else(
                |_|
                    panic!(
                        "Unable to build window!"
                    )
            )
    };

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let home_dir = match dirs::home_dir() {
        Some(path1) => path1,
        None => panic!("Error, can't find the home directory!!"),
    };

    #[cfg(target_os = "windows")]
    let data_dir = home_dir.join("AppData").join("Roaming").join(APP_NAME);
    
    #[cfg(target_os = "linux")]
    let data_dir = home_dir.join(".config").join(APP_NAME);
    
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir)
            .unwrap_or_else(
                |_| 
                    panic!("Can't create dir {}", data_dir.display())
            );
    }
    
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let mut web_context = WebContext::new(Some(data_dir));

    #[allow(unused_mut)]
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let mut _webview = Some(
        WebViewBuilder::new(window)?
            .with_user_agent(&user_agent)
            .with_accept_first_mouse(true)
            .with_transparent(true)
            .with_devtools(cfg!(any(debug_assertions, feature = "devtools")))
            .with_url(&DISCORD.to_string())?
            .with_web_context(&mut web_context)
            .build()?,
    );

    #[cfg(feature = "devtools")]
    {
        _webview.open_devtools();
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Discord web view successfully started."),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::UserEvent(UserEvents::CloseWindow) => {
                let _ = _webview.take();
                *control_flow = ControlFlow::Exit
            }
            _ => (),
        }
    });
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn load_icon(path: &std::path::Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path.")
            .into_rgba8();

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon.")
}
