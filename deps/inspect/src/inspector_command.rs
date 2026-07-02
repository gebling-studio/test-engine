use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InspectorCommand {
    PlaySound,
    UI(UIRequest),
}

impl From<UIRequest> for InspectorCommand {
    fn from(value: UIRequest) -> Self {
        Self::UI(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIRequest {
    SetScale(f32),
    GetUI,
    EditRule {
        view_id:    String,
        rule_index: usize,
        offset:     f32,
        enabled:    bool,
    },
}
