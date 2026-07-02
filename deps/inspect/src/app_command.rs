use refs::Own;
use serde::{Deserialize, Serialize};

use crate::ui::ViewRepr;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppCommand {
    Ok,
    UI(UIResponse),
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
