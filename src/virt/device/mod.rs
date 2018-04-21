pub mod virtio;

use super::{Instance, Run, Action};
use error::*;

pub trait Device {
    fn initialize(&mut self, instance: &mut Instance) -> Result<()>;
    fn interrupt(&mut self, instance: &mut Instance, run: &mut Run) -> Action;
}
