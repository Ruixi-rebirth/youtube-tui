use tui::layout::Constraint;

use crate::app::pages::{global::GlobalItem, main_menu::MainMenuItem, item_info::ItemInfoItem};

#[derive(Debug, Clone)]
pub enum Item {
    Global(GlobalItem),
    MainMenu(MainMenuItem),
    ItemInfo(ItemInfoItem),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub items: Vec<RowItem>,
    pub centered: bool,
    pub height: Constraint,
}

#[derive(Debug, Clone)]
pub struct RowItem {
    pub item: Item,
    pub constraint: Constraint,
}