use super::log;
use strum::IntoEnumIterator;

#[test]
fn test_log() {
    for priority in log::Priority::iter() {
        log::write(format!( "{:?}", priority ), log::Priority::Info);
        log::write("test_log: test logging capability".to_string(), priority);
    }
    
}
