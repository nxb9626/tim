use tokio::sync::Mutex;

use gui::shape::Shapes;

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock};

use crate::crosshair::Crosshair;
use crate::debug::Debugger;

pub mod crosshair;
pub mod debug;

pub type Objects = Arc<Mutex<BTreeMap<usize, BTreeMap<String, Component>>>>;

pub static COMPONENT_LAYERS: LazyLock<Arc<Mutex<BTreeMap<usize, BTreeMap<String, Component>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(BTreeMap::new())));

pub enum Component {
    Shapes(Shapes),
    Debugger(Debugger),
    Crosshair(Crosshair),
}

impl From<Shapes> for Component {
    fn from(value: Shapes) -> Self {
        Component::Shapes(value)
    }
}

impl From<Debugger> for Component {
    fn from(value: Debugger) -> Self {
        Component::Debugger(value)
    }
}

impl From<Crosshair> for Component {
    fn from(value: Crosshair) -> Self {
        Component::Crosshair(value)
    }
}
