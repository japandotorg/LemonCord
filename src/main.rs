extern crate wry;
extern crate dirs;
extern crate image;
extern crate tokio;
extern crate anyhow;

mod log;

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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    log::write(format!("LemonCord : {:?}", &VERSION.to_string().to_owned()), log::Priority::Info);

    if discord().await.is_err() {
        log::write(
            "Fatal error while creating discord window. shutting down.".to_string(),
            log::Priority::High
        );
    }
}

async fn discord() -> Result<()> {
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

    let event_loop: EventLoop<UserEvents> = EventLoop::<UserEvents>::with_user_event();

    let main_window: WindowBuilder = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_transparent(true)
        .with_resizable(true);

    #[cfg(target_os = "windows")]
    let window: Window = {
        let mut icon_path: String = format!("assets/{}_logo.webp", APP_NAME);

        if !std::path::Path::new(&icon_path).exists() {
            icon_path = "assets/logo.webp".to_string();
        }

        let icon: Icon = load_icon(std::path::Path::new(&icon_path));

        main_window
            .with_window_icon(Some(icon.clone()))
            .with_taskbar_icon(Some(icon.clone()))
            .build(&event_loop)
            .unwrap_or_else(
                |_| {
                    log::write(
                        "Unable to build window, shutting down!".to_string(),
                        log::Priority::High
                    );
                    std::process::exit(1);
                }
            )
    };

    #[cfg(target_os = "linux")]
    let window: Window = {
        let mut icon_path: String = format!("assets/{}_logo.webp", APP_NAME);

        if !std::path::Path::new(&icon_path).exists() {
            icon_path = "assets/logo.webp".to_string();
        }

        let icon: Icon = load_icon(std::path::Path::new(&icon_path));

        main_window
            .with_window_icon(Some(icon.clone()))
            .build(&event_loop)
            .unwrap_or_else(
                |_| {
                    log::write(
                        "Unable to build window, shutting down!".to_string(),
                        log::Priority::High
                    );
                    std::process::exit(1);
                }
            )
    };

    #[cfg(target_os = "macos")]
    let window: Window = {
        main_window
            .with_fullsize_content_view(true)
            .with_titlebar_buttons_hidden(false)
            .with_titlebar_transparent(true)
            .with_title_hidden(false)
            .with_menu(menu_bar_menu)
            .build(&event_loop)
            .unwrap_or_else(
                |_| {
                    log::write(
                        "Unable to build window, shutting down!".to_string(),
                        log::Priority::High
                    );
                    std::process::exit(1);
                }
            )


    };

    let _handler = move |window: &Window, req: String| {
        if req == "drag_window" {
            let _ = window.drag_window();
        } else if req == "fullscreen" {
            let is_maximized: bool = window.is_maximized();
            window.set_maximized(!is_maximized);
        } else if let Some(list) = req.strip_prefix("branches:") {
            save_branches(list);
        } else if let Some(host) = req.strip_prefix("host:") {
            save_host(host);
        }
    };

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let home_dir = match dirs::home_dir() {
        Some(path1) => path1,
        None => {
            log::write(
                "Unable to locate your HOME directory. Shutting down, this should NEVER happen.".to_string(),
                log::Priority::High
            );
            std::path::PathBuf::new() // We are still shutting down. The rust compiler doesn't understand this and throws `match arms` panic.
        },
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
                    log::write(
                        format!( "Can't create dir {} Config files cannot save.", data_dir.display() ),
                        log::Priority::Medium
                    )
            );
    }
    
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let mut web_context = WebContext::new(Some(data_dir));

    let mods_script = load_mods();

    #[allow(unused_mut)]
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let mut _webview: Option< wry::webview::WebView > = {
        #[cfg(target_os = "windows")]
        let user_agent: String = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();
        
        #[cfg(target_os = "linux")]
        let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();
        Some (
            WebViewBuilder::new(window)?
                .with_user_agent(&user_agent)
                .with_accept_first_mouse(true)
                .with_transparent(true)
                .with_devtools(cfg!(any(debug_assertions, feature = "devtools")))
                .with_initialization_script(&mods_script)
                .with_ipc_handler(_handler)
                .with_url(&DISCORD.to_string())?
                .with_web_context(&mut web_context)
                .build()?
        )
    };

    #[allow(unused_mut)]
    #[cfg(target_os = "macos")]
    let _webview: Option<wry::webview::WebView> = {
        let user_agent: String = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15".to_string();
        
        Some(
            WebViewBuilder::new(window)?
                .with_user_agent(&user_agent)
                .with_initialization_script(&mods_script)
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

    event_loop.run(move |event: Event<UserEvents>, _, control_flow: &mut ControlFlow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => log::write(
                "Discord web view successfully started.".to_string(),
                log::Priority::Info
            ),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::UserEvent(UserEvents::CloseWindow) => {
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
                log::write(
                    format!( "Clicked on {:?}", menu_id),
                    log::Priority::Info
                );
            }
            _ => (),
        }
    });
}

/// Mods LemonCord downloads, by sheltupdate branch name, with their browser builds.
/// Using sheltupdate's branch names keeps shelter-installer's settings.json entry working unchanged.
const MODS: [(&str, &str); 3] = [
    ("shelter", "https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js"),
    ("vencord", "https://github.com/Vendicated/Vencord/releases/download/devbuild/Vencord.user.js"),
    ("equicord", "https://github.com/Equicord/Equicord/releases/download/latest/Equicord.user.js"),
];

/// sheltupdate tweaks that are plain scripts run inside embed iframes, copied from sheltupdate
/// (see src/sheltupdate/VERSION). The rest of sheltupdate's branches need Electron.
const TWEAKS: [(&str, &str, &str); 2] = [
    ("spotify_embed_volume", "https://open.spotify.com/embed/", include_str!("sheltupdate/spotify-embed-volume-script.js")),
    ("yt_ad_block", "https://www.youtube.com/embed/", include_str!("sheltupdate/yt-ad-block-script.js")),
];

const DEFAULT_HOST: &str = "https://inject.shelter.uwu.network";

fn supported_branches() -> Vec<&'static str> {
    MODS.iter().map(|(n, _)| *n).chain(TWEAKS.iter().map(|(n, _, _)| *n)).collect()
}

/// Same folder shelter-installer resolves with Go's os.UserConfigDir().
fn config_dir() -> std::path::PathBuf {
    dirs::config_dir().unwrap_or_default().join(APP_NAME)
}

fn read_settings() -> serde_json::Map<String, serde_json::Value> {
    std::fs::read_to_string(config_dir().join("settings.json"))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Splits an endpoint like "https://host/shelter+vencord/" into host and branches, matching
/// sheltupdate's own /^(https?:\/\/.+)\/([a-zA-Z0-9_+-]+)\/?$/.
fn parse_endpoint(endpoint: &str) -> Option<(String, Vec<String>)> {
    let trimmed = endpoint.strip_suffix('/').unwrap_or(endpoint);
    let (host, branches) = trimmed.rsplit_once('/')?;
    let valid_host = host
        .split_once("://")
        .map_or(false, |(scheme, rest)| (scheme == "https" || scheme == "http") && !rest.is_empty());
    let valid_branches = !branches.is_empty()
        && branches.chars().all(|c| c.is_ascii_alphanumeric() || "_+-".contains(c));

    (valid_host && valid_branches).then(|| (host.to_string(), branches.split('+').map(String::from).collect()))
}

/// shelter-installer and sheltupdate turn mods on by pointing UPDATE_ENDPOINT at a sheltupdate
/// host with a branch list. LemonCord has no Discord updater to proxy, so it only reads that list.
fn current_endpoint() -> (String, Vec<String>) {
    let settings = read_settings();
    ["UPDATE_ENDPOINT", "NEW_UPDATE_ENDPOINT"]
        .iter()
        .filter_map(|key| settings.get(*key).and_then(|v| v.as_str()).and_then(parse_endpoint))
        .next()
        .unwrap_or_else(|| (DEFAULT_HOST.to_string(), Vec::new()))
}

/// Writes the endpoints the same way sheltupdate's SHELTER_BRANCH_SET does.
fn write_endpoint(host: &str, branches: &[&str]) {
    let mut settings = read_settings();

    if branches.is_empty() {
        settings.remove("UPDATE_ENDPOINT");
        settings.remove("NEW_UPDATE_ENDPOINT");
    } else {
        let endpoint = format!("{}/{}", host, branches.join("+"));
        settings.insert("NEW_UPDATE_ENDPOINT".to_string(), format!("{}/", endpoint).into());
        settings.insert("UPDATE_ENDPOINT".to_string(), endpoint.into());
    }

    let _ = std::fs::create_dir_all(config_dir());
    let written = serde_json::to_string_pretty(&settings)
        .map_err(|e| e.to_string())
        .and_then(|json| std::fs::write(config_dir().join("settings.json"), json).map_err(|e| e.to_string()));
    if let Err(e) = written {
        log::write(format!("Couldn't save client mods: {}", e), log::Priority::Medium);
    }
}

/// IPC from the Client Mods page. Page scripts can send this too, so only branches LemonCord or the
/// sheltupdate server know about are accepted. Unsupported ones are saved but never loaded.
fn save_branches(list: &str) {
    let server: Vec<serde_json::Value> = std::fs::read_to_string(config_dir().join("sheltupdate_branches.json"))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    let known = |b: &str| supported_branches().contains(&b) || server.iter().any(|s| s["name"] == b);

    let branches: Vec<&str> = list.split('+').filter(|b| !b.is_empty()).collect();
    if let Some(bad) = branches.iter().find(|b| !known(b)) {
        log::write(format!("Rejected unknown client mod {:?}.", bad), log::Priority::Medium);
        return;
    }
    write_endpoint(&current_endpoint().0, &branches);
}

/// IPC from the Client Mods page's "Change" instance button.
fn save_host(host: &str) {
    let has_path = host.split_once("://").map_or(true, |(_, rest)| rest.contains('/'));
    if has_path || parse_endpoint(&format!("{}/shelter", host)).is_none() {
        log::write(format!("Rejected sheltupdate host {:?}.", host), log::Priority::Medium);
        return;
    }

    // Like sheltupdate, the host lives inside the endpoint, so there's nothing to save without branches.
    let (_, branches) = current_endpoint();
    let branches: Vec<&str> = branches.iter().map(String::as_str).collect();
    if !branches.is_empty() {
        write_endpoint(host, &branches);
    }
}

#[cfg(test)]
mod tests {
    use super::parse_endpoint;

    #[test]
    fn parses_sheltupdate_endpoints() {
        let parsed = parse_endpoint("https://inject.shelter.uwu.network/shelter+vencord/");
        assert_eq!(parsed, Some(("https://inject.shelter.uwu.network".to_string(), vec!["shelter".to_string(), "vencord".to_string()])));
        assert_eq!(parse_endpoint("http://localhost:8080/shelter").map(|p| p.0), Some("http://localhost:8080".to_string()));
        assert_eq!(parse_endpoint("https://inject.shelter.uwu.network"), None);
        assert_eq!(parse_endpoint("https://host/shel ter"), None);
        assert_eq!(parse_endpoint("ftp://host/shelter"), None);
    }
}

/// Downloads `url` in the background with curl. Pair with `finish_download`.
// HACK: shells out to curl (bundled with Windows 10+, macOS and most Linux) to avoid an HTTP crate; swap for ureq if curl is ever missing.
fn start_download(url: &str) -> Option<std::process::Child> {
    std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "20", url])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()
}

/// Waits for a download and caches it, falling back to the last cached copy so an outage on a
/// mod's side doesn't stop LemonCord from opening. Returns "" if neither exists.
fn finish_download(child: Option<std::process::Child>, cache_name: &str) -> String {
    let cache = config_dir().join(cache_name);
    let fetched = child
        .and_then(|c| c.wait_with_output().ok())
        .filter(|out| out.status.success() && !out.stdout.is_empty())
        .and_then(|out| String::from_utf8(out.stdout).ok());

    match fetched {
        Some(text) => {
            let _ = std::fs::create_dir_all(config_dir());
            let _ = std::fs::write(&cache, &text);
            text
        }
        None => {
            log::write(format!("Couldn't download {}, using cached copy.", cache_name), log::Priority::Medium);
            std::fs::read_to_string(&cache).unwrap_or_default()
        }
    }
}

/// Builds the init script for every enabled branch. Returns "" when none are on.
fn load_mods() -> String {
    let (host, current) = current_endpoint();
    let supported = supported_branches();
    let enabled: Vec<&str> = supported.iter().copied().filter(|b| current.iter().any(|c| c == b)).collect();
    if enabled.is_empty() {
        return String::new();
    }

    // All downloads start at once because Equicord alone is ~3 MB.
    let mod_downloads: Vec<_> = MODS
        .iter()
        .filter(|(name, _)| enabled.contains(name))
        .map(|(name, url)| (*name, start_download(url)))
        .collect();
    let branch_list = enabled
        .contains(&"shelter")
        .then(|| start_download(&format!("{}/sheltupdate_branches", host)));

    let mut discord = String::new();

    if let Some(download) = branch_list {
        let available: serde_json::Value =
            serde_json::from_str(&finish_download(download, "sheltupdate_branches.json")).unwrap_or_else(|_| serde_json::json!([]));
        let state = serde_json::json!({ "branches": current, "host": host, "available": available, "supported": supported });
        // Hands sheltupdate's own Client Mods page to shelter as a built-in plugin, as sheltupdate's preload.js does.
        let injector_plugins = serde_json::json!({
            "sheltupdate-branch-selector": {
                "js": include_str!("sheltupdate/selector-ui.js"),
                "manifest": {
                    "name": "sheltupdate branch selector",
                    "author": "uwu.network",
                    "description": "responsible for the 'Client Mods' UI on sheltupdate installs",
                },
                "injectorIntegration": { "isVisible": false, "allowedActions": {}, "loaderName": "sheltupdate" },
            }
        });
        discord.push_str(&format!(
            "window.__lemoncord = {};\n{}\nconst SHELTER_INJECTOR_PLUGINS = {};\n",
            state,
            include_str!("sheltupdate/native.js"),
            injector_plugins
        ));
    }

    for (name, download) in mod_downloads {
        let bundle = finish_download(download, &format!("{}.js", name));
        if bundle.is_empty() {
            log::write(format!("No {} build available, starting without it.", name), log::Priority::Medium);
        } else if name == "shelter" {
            discord.push_str(&format!("{}\n", bundle));
        } else {
            discord.push_str(&format!("(function () {{\n{}\n{}\n}})();\n", include_str!("userscript_shim.js"), bundle));
        }
    }

    // Init scripts run in every frame: mods go in Discord's top-level page, tweaks in their embeds.
    let mut script = format!("if (window.top === window && location.hostname === \"discord.com\") {{\n{}\n}}\n", discord);
    for (name, prefix, tweak) in TWEAKS.iter().filter(|(name, _, _)| enabled.contains(name)) {
        script.push_str(&format!(
            "// {}\nif (location.href.startsWith({:?})) document.addEventListener(\"DOMContentLoaded\", () => {{\n{}\n}});\n",
            name, prefix, tweak
        ));
    }
    script
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn load_icon(path: &std::path::Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path.")
            .into_rgba8();

        let (width, height) = image.dimensions();
        let rgba: Vec<u8> = image.into_raw();
        (rgba, width, height)
    };

    let result = Icon::from_rgba(icon_rgba, icon_width, icon_height);
    if result.is_err() {
        log::write(
            "Failed to open icon, shutting down.".to_string(),
            log::Priority::High
        );
    }
    result.unwrap()
}
