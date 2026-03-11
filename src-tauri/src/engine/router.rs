use super::{EngineId, SttEngine};
use std::collections::HashMap;
use std::sync::Arc;

pub struct EngineRouter {
    engines: HashMap<EngineId, Arc<dyn SttEngine>>,
    default_engine: EngineId,
}

impl EngineRouter {
    pub fn new(default_engine: EngineId) -> Self {
        Self {
            engines: HashMap::new(),
            default_engine,
        }
    }

    pub fn register(&mut self, engine: Arc<dyn SttEngine>) {
        self.engines.insert(engine.id(), engine);
    }

    pub fn get(&self, id: &EngineId) -> Option<Arc<dyn SttEngine>> {
        self.engines.get(id).cloned()
    }

    pub fn get_default(&self) -> Option<Arc<dyn SttEngine>> {
        self.get(&self.default_engine)
    }

    pub fn available_engines(&self) -> Vec<EngineId> {
        self.engines.keys().cloned().collect()
    }
}
