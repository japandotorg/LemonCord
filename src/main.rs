extern crate wry;
extern crate dirs;
extern crate image;
extern crate tokio;
extern crate anyhow;


use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, Window, Icon}, 
        menu::MenuType,
    },
    webview::WebViewBuilder,
};

#[cfg(target_os = "macos")]
use wry::application::{
    accelerator::{Accelerator, SysMods},
    keyboard::KeyCode,
    menu::{MenuBar as Menu, MenuItem, MenuItemAttributes},
    platform::macos::WindowBuilderExtMacOS,
};

#[cfg(target_os = "windows")]
use wry::application::platform::windows::WindowBuilderExtWindows;

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


#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(target_os = "macos")]
    let (menu_bar_menu, close_item) = {
        let mut menu_bar_menu = Menu::new();
        let mut first_menu = Menu::new();
        first_menu.add_native_item(MenuItem::Hide);
        first_menu.add_native_item(MenuItem::EnterFullScreen);
        first_menu.add_native_item(MenuItem::Minimize);
        first_menu.add_native_item(MenuItem::Separator);
        first_menu.add_native_item(MenuItem::Copy);
        first_menu.add_native_item(MenuItem::Cut);
        first_menu.add_native_item(MenuItem::Paste);
        first_menu.add_native_item(MenuItem::Undo);
        first_menu.add_native_item(MenuItem::Redo);
        first_menu.add_native_item(MenuItem::SelectAll);
        first_menu.add_native_item(MenuItem::Separator);
        let close_item = first_menu.add_item(
            MenuItemAttributes::new("CloseWindow")
                .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyW)),
        );
        first_menu.add_native_item(MenuItem::Quit);
        menu_bar_menu.add_submenu("App", true, first_menu);
        (menu_bar_menu, close_item)
    };

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
            .with_menu(menu_bar_menu)
            .build(&event_loop)
            .unwrap_or_else(
                |_|
                    panic!(
                        "Unable to build window!"
                    )
            )
    };

    let _handler = move |window: &Window, req: String| {
        if req == "drag_window" {
            let _ = window.drag_window();
        } else if req == "fullscreen" {
            let is_maximuzed = window.is_maximized();
            window.set_maximized(!is_maximuzed);
        }
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
    let mut _webview = {
        #[cfg(target_os = "windows")]
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();
        
        #[cfg(target_os = "linux")]
        let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();
        
        Some(
            WebViewBuilder::new(window)?
                .with_user_agent(&user_agent)
                .with_accept_first_mouse(true)
                .with_transparent(true)
                .with_devtools(cfg!(any(debug_assertions, feature = "devtools")))
                .with_url(&DISCORD.to_string())?
                .with_web_context(&mut web_context)
                .build()?,
        )
    };

    #[allow(unused_mut)]
    #[cfg(target_os = "macos")]
    let _webview = {
        let user_agent_string = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15".to_string();
        
        Some(
            WebViewBuilder::new(window)?
                .with_user_agent(&user_agent)
                .with_url(&DISCORD.to_string())?
                .with_devtools(cfg!(any(debug_assertions, feature = "devtools")))
                .with_ipc_handler(_handler)
                .with_back_forward_navigation_gestures(true)
                .build()?
        )
    };

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
            | Event::MenuEvent {
                menu_id,
                origin: MenuType::MenuBar,
                ..
            } => {
                #[cfg(target_os = "macos")]
                if menu_id == close_item.clone().id() {
                    window.set_minimized(true)
                }
                println!("Clicked on {:?}", menu_id);
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
