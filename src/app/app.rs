use std::collections::LinkedList;

use crate::{
    structs::{AppHistory, Item, Page, SearchSettings, State, WatchHistory, MessageText},
    traits::ItemTrait,
};
use crossterm::event::KeyEvent;
use invidious::blocking::Client;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use super::config::{Action, Config, LayoutConfig};

#[derive(Clone)]
pub struct App {
    pub config: Config,
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
    pub history: Vec<AppHistory>,
    pub watch_history: WatchHistory,
    pub search_settings: SearchSettings,
    pub search_text: String,
    pub search_index: usize,
    pub page_no: usize,
    pub term_clear: bool,
}

pub struct AppNoState {
    pub config: Config,
    pub page: Page,
    pub selectable: Vec<Vec<(usize, usize)>>,
    pub hover: Option<(usize, usize)>, // x, y
    pub selected: Option<(usize, usize)>,
    pub client: Client,
    pub message: MessageText,
    pub load: bool,
    pub render: bool,
    pub popup_focus: bool,
    pub history: Vec<AppHistory>,
    pub watch_history: WatchHistory,
    pub search_settings: SearchSettings,
    pub search_text: String,
    pub search_index: usize,
    pub page_no: usize,
    pub term_clear: bool,
}

impl AppNoState {
    pub fn split(app: App) -> (Self, State) {
        let out = Self {
            config: app.config,
            page: app.page,
            selectable: app.selectable,
            hover: app.hover,
            selected: app.selected,
            client: app.client,
            message: app.message,
            load: app.load,
            render: app.render,
            popup_focus: app.popup_focus,
            history: app.history,
            watch_history: app.watch_history,
            search_settings: app.search_settings,
            search_text: app.search_text,
            search_index: app.search_index,
            page_no: app.page_no,
            term_clear: app.term_clear,
        };
        (out, app.state)
    }

    pub fn join(app: Self, state: State) -> App {
        App {
            config: app.config,
            page: app.page,
            selectable: app.selectable,
            hover: app.hover,
            selected: app.selected,
            client: app.client,
            message: app.message,
            load: app.load,
            render: app.render,
            popup_focus: app.popup_focus,
            history: app.history,
            watch_history: app.watch_history,
            search_settings: app.search_settings,
            search_text: app.search_text,
            search_index: app.search_index,
            page_no: app.page_no,
            state,
            term_clear: app.term_clear,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let config = Config::load().unwrap();
        let state: State = config.layouts.main_menu.clone().into();
        let selectable = App::selectable(&state);
        Self {
            page: Page::default(),
            state,
            selectable,
            client: Client::new(config.main.server_url.clone()),
            selected: None,
            hover: None,
            message: MessageText::None,
            load: true,
            render: true,
            popup_focus: false,
            history: Vec::new(),
            config,
            watch_history: WatchHistory::load(),
            search_settings: SearchSettings::default(),
            search_text: String::new(),
            search_index: 0,
            page_no: 1,
            term_clear: false,
        }
    }
}

impl App {
    pub fn selectable(state: &State) -> Vec<Vec<(usize, usize)>> {
        let mut selectable = Vec::new();

        for (y, row) in state.0.iter().enumerate() {
            let mut row_vec = Vec::new();
            for (x, row_item) in row.items.iter().enumerate() {
                if match &row_item.item {
                    Item::Global(item) => item.selectable(),
                    Item::MainMenu(item) => item.selectable(),
                    Item::ItemInfo(item) => item.selectable(),
                    Item::Search(item) => item.selectable(),
                    Item::Channel(item) => item.selectable(),
                } {
                    row_vec.push((x, y));
                }
            }
            if row_vec.len() != 0 {
                selectable.push(row_vec);
            }
        }

        selectable
    }

    pub fn key_input(mut self, key: KeyEvent) -> App {
        let action = self.config.keybindings.0.get(&key);

        if let Some((x, y)) = self.selected {
            if action != Some(&Action::Deselect) {
                let mut item = self.state.0[y].items[x].item.clone();
                let updated;
                (updated, self) = item.key_input(key, self);
                if updated {
                    self.state.0[y].items[x].item = item;
                }

                return self;
            }
        }

        let action = match action {
            Some(action) => *action,
            None => return self,
        };

        match action {
            Action::Refresh => {
                self.state.reset();
                self.load = true;
                self.render = true;
            }
            Action::Select => {
                if self.hover.is_some() && self.selected.is_none() {
                    let (mut x, mut y) = self.hover.unwrap();
                    (x, y) = self.selectable[y][x];

                    let select;
                    (self, select) = self.state.0[y]
                        .items
                        .iter()
                        .nth(x)
                        .clone()
                        .unwrap()
                        .item
                        .clone()
                        .select(self);
                    if select {
                        self.selected = Some((x, y));
                    }

                    self.render = true;

                    return self;
                }
            }
            Action::Deselect => {
                if self.selected.is_some() {
                    self.selected = None;
                    self.popup_focus = false;
                    self.render = true;
                    return self;
                }
            }

            _ => {}
        }

        match &mut self.hover {
            Some((x, y)) => match action {
                Action::Up => {
                    if *y > 0 {
                    self.render = true;
                        let temp_y = *y - 1;
                        if *x > self.selectable[temp_y].len() {
                            let temp_x = self.selectable[temp_y].len();
                            if temp_x > self.selectable[*y].len() - 1 {
                                *x = self.selectable[*y].len() - 1;
                            }
                        }
                        *y -= 1;
                        if *x > self.selectable[*y].len() - 1 {
                            *x = self.selectable[*y].len() - 1;
                        }
                    }
                }
                Action::Down => {
                    if *y < self.selectable.len() - 1 {
                    self.render = true;
                        *y += 1;
                        if *x > self.selectable[*y].len() - 1 {
                            *x = self.selectable[*y].len() - 1;
                        }
                    }
                }

                Action::Left => {
                    if *x > 0 {
                    self.render = true;
                        *x -= 1;
                    }
                }

                Action::Right => {
                    if *x < self.selectable[*y].len() - 1 {
                    self.render = true;
                        *x += 1;
                    }
                }

                _ => {}
            },
            None => match action {
                Action::Up => {
                    self.render = true;
                    self.hover = Some((0, 0));
                }
                Action::Down => {
                    self.render = true;
                    self.hover = Some((0, self.selectable.len() - 1));
                }
                Action::Left => {
                    self.render = true;
                    self.hover = Some((0, 0));
                }
                Action::Right => {
                    self.render = true;
                    self.hover = Some((0, self.selectable.len() - 1));
                }
                _ => {}
            },
        }

        self
    }

    pub fn render<B: Backend>(mut self, frame: &mut Frame<B>) -> Self {
        let size = frame.size();
        let mut popups = Vec::new();

        let min = self.page.min(&self.config);

        if size.width < min.0 || size.height < min.1 {
            let paragraph = Paragraph::new(format!(
                "Window too small. Minimum size for this page is {} x {}. Current size is {} x {}",
                min.0, min.1, size.width, size.height
            ))
            .block(Block::default())
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, size);
            return self;
        }

        let hover_selected = if let Some((x, y)) = self.hover {
            Some(self.selectable[y][x])
        } else {
            None
        };

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.state
                    .0
                    .iter()
                    .map(|row| row.height)
                    .collect::<Vec<Constraint>>(),
            )
            .split(size);

        let (mut app, mut state) = AppNoState::split(self);

        for (y, (row, row_chunk)) in state
            .0
            .iter_mut()
            .zip(vertical_chunks.clone().iter_mut())
            .enumerate()
        {
            let mut constraints = LinkedList::new();
            let mut length = match row.centered {
                true => Some(0),
                false => None,
            };
            for item in row.items.iter() {
                constraints.push_back(item.constraint);
                if let Some(length_value) = length {
                    length = Some(match item.constraint {
                        Constraint::Length(l) | Constraint::Max(l) | Constraint::Min(l) => {
                            l + length_value
                        }
                        Constraint::Percentage(p) => length_value + size.width * p / 100,
                        _ => unreachable!(),
                    })
                }
            }

            if let Some(i) = length {
                let extra_constraint = Constraint::Length((size.width - i) / 2);
                constraints.push_front(extra_constraint);
            } else {
                constraints.push_front(Constraint::Length(0));
            }

            constraints.push_back(Constraint::Length(0));

            let mut chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.into_iter().collect::<Vec<Constraint>>())
                .split(*row_chunk)
                .into_iter();

            frame.render_widget(Block::default(), chunks.next().unwrap());

            let popup_focus = app.popup_focus;
            for (x, (chunk, item)) in chunks
                .zip(row.items.iter_mut().map(|i| &mut i.item))
                .enumerate()
            {
                let selected = app.selected == Some((x, y));

                let hover = hover_selected == Some((x, y));

                let hold = item.render_item(frame, chunk, app, selected, hover, popup_focus, false);

                app = hold.1;

                if hold.0 {
                    popups.push((item, selected, hover, chunk));
                }
            }
        }

        for (item, selected, hover, chunk) in popups {
            let hold = item.render_item(frame, chunk, app, selected, hover, true, true);
            app = hold.1;
        }

        self = AppNoState::join(app, state);
        self
    }

    pub fn pop(mut self) -> (App, bool) {
        if self.history.len() == 0 {
            self.message = MessageText::Text(String::from("This is the beginning of history"));
            return (self, false);
        }

        let app_history = self.history.pop().unwrap();

        self = Self {
            state: app_history.state,
            selectable: app_history.selectable,
            selected: app_history.selected,
            hover: app_history.hover,
            message: app_history.message,
            page: app_history.page,
            client: app_history.client,
            load: app_history.load,
            render: app_history.render,
            history: self.history.clone(),
            config: self.config,
            watch_history: self.watch_history,
            search_settings: self.search_settings,
            popup_focus: app_history.popup_focus,
            search_text: app_history.search_text,
            search_index: app_history.search_index,
            page_no: app_history.page_no,
            term_clear: false,
        };

        (self, true)
    }

    pub fn home(&mut self) {
        *self = App::default();
    }

    pub fn page_layout(&self) -> LayoutConfig {
        match self.page {
            Page::Search => self.config.layouts.search.clone(),
            Page::MainMenu(_) => self.config.layouts.main_menu.clone(),
            Page::ItemDisplay(_) => self.config.layouts.item_info.clone(),
            Page::Channel(_, _) => self.config.layouts.channel.clone(),
        }
    }

    pub fn page_default(&self) -> Option<(usize, usize)> {
        match self.page {
            Page::Search => self.config.layouts.search.def_selected, 
            Page::MainMenu(_) => self.config.layouts.main_menu.def_selected, 
            Page::ItemDisplay(_) => self.config.layouts.item_info.def_selected, 
            Page::Channel(_, _) => self.config.layouts.channel.def_selected, 
        }
    }
}
