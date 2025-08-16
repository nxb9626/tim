use tokio::sync::Mutex;

use gui::shape::Shapes;

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock};

use crate::crosshair::Crosshair;
use crate::debug::Debugger;

pub mod crosshair;
pub mod debug;

pub type Objects = Arc<Mutex<BTreeMap<usize, BTreeMap<String, Component>>>>;

pub static NOT_RENDERED: usize = 0;
pub static BACKGROUND: usize = 1;
pub static FOREGROUND: usize = 3;
pub static DEBUGGER: usize = 4;

pub static COMPONENT_LAYERS: LazyLock<Arc<Mutex<BTreeMap<usize, BTreeMap<String, Component>>>>> =
    LazyLock::new(|| {
        // Initialize layer counter
        // let mut layer_count = LayerTracker { id: 0 };

        let mut layers: BTreeMap<usize, BTreeMap<String, Component>> = BTreeMap::new();

        // initialize layers
        layers.insert(NOT_RENDERED, BTreeMap::new());
        layers.insert(BACKGROUND, BTreeMap::new());
        layers.insert(FOREGROUND, BTreeMap::new());
        layers.insert(DEBUGGER, BTreeMap::new());

        // make shareable
        Arc::new(Mutex::new(layers))
    });

#[derive(Debug)]
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
