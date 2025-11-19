pub mod html;
pub mod terminal;

use crate::types::ErrorGroup;
use crate::Result;

pub trait OutputFormatter {
    fn format(&self, groups: &[ErrorGroup]) -> Result<String>;
}
