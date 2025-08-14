use gui::shape::Shapes;

use crate::crosshair::Crosshair;
use crate::debug::Debugger;

pub mod crosshair;
pub mod debug;

pub enum Component {
    Shapes(Shapes),
    DebugMenu(Debugger),
    Crosshair(Crosshair),
}
