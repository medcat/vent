use super::Device;
use std::sync::Arc;

pub mod e9;
pub mod sconsole;

pub fn default() -> Vec<Arc<Device>> {
    vec![
        Arc::new(e9::E9::new(None)),
        Arc::new(e9::E9::new(Some(0x80))),
        Arc::new(sconsole::SerialConsole::new(0x3f8)),
    ]
}
