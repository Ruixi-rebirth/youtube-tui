use invidious::blocking::Client;

use crate::{app::app::App, structs::Page};

use super::{SearchSettings, State, MessageText};

#[derive(Clone)]
pub struct AppHistory {
    pub page: Page,
    pub state: State, // Item
    pub selectable: Vec<Vec<(usize, usize)>>,
    pub hover: Option<(usize, usize)>, // x, y
    pub selected: Option<(usize, usize)>,
    pub client: Client,
    pub message: MessageText,
    pub load: bool,
    pub render: bool,
    pub popup_focus: bool,
    pub search_text: String,
    pub search_settings: SearchSettings,
    pub search_index: usize,
    pub page_no: usize,
}

impl From<App> for AppHistory {
    fn from(original: App) -> Self {
        Self {
            page: original.page,
            state: original.state,
            selectable: original.selectable,
            hover: original.hover,
            // selected: original.selected,
            selected: original.selected,
            client: original.client,
            message: original.message,
            load: original.load,
            render: original.render,
            popup_focus: original.popup_focus,
            search_text: original.search_text,
            search_settings: original.search_settings,
            search_index: original.search_index,
            page_no: original.page_no,
        }
    }
}
