use refs::Own;
use serde::{Deserialize, Serialize};

use crate::inspect::protocol::ui::ViewRepr;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppCommand {
    Ok,
    Error(String),
    Screenshot {
        width:      u32,
        height:     u32,
        png_base64: String,
    },
    Edits(Vec<EditEntry>),
    UI(UIResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditEntry {
    pub timestamp: String,
    pub view:      String,
    pub view_id:   String,
    pub what:      String,
    pub old:       String,
    pub new:       String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UIResponse {
    SendUI { scale: f32, root: Own<ViewRepr> },
}

impl From<UIResponse> for AppCommand {
    fn from(value: UIResponse) -> Self {
        Self::UI(value)
    }
}
