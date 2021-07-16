use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DataVec {
    /// Generic JSON array representation
    dataset: Vec<Value>,
}

impl DataVec {
    /// Get a reference to the data vec's dataset.
    pub fn dataset(&self) -> &[Value] {
        self.dataset.as_slice()
    }
}
