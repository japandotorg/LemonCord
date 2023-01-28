use colored::Colorize;
use strum_macros::EnumIter;

/// Log writing priority
///
/// High: RED
///
/// Medium: Bright yellow
///
/// Low: Fading yellow
///
/// Info: Purple
#[derive(EnumIter, Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
    Info

}
/// Write to log ( stdout )
pub fn write(message: String, priority: Priority) {
    match priority {
        Priority::High => {
            println!("[ERROR]: {}\n", message.red())
        }

        Priority::Medium => {
            println!("[ERROR]: {}\n", message.truecolor(255, 255, 0))
        }

        Priority::Low => {
            println!("[ERROR]: {}\n", message.truecolor(177, 177, 0) )
        }

        Priority::Info => {
            println!("[LOG]: {}\n", message.truecolor(170, 62, 255))
        }
    }
}