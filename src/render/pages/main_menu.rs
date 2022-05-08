use crate::{
    app::{pages::main_menu::{MainMenuItem, MainMenuSelector}, app::Page},
    traits::RenderItem,
    widgets::horizontal_split::HorizontalSplit,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

impl MainMenuItem {
    pub fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
        page: &Page
    ) {
        let mut style = Style::default().fg(if selected {
            Color::LightBlue
        } else if hover {
            Color::LightRed
        } else {
            Color::Reset
        });

        match self {
            MainMenuItem::SeletorTab(selector) => {
                if !hover && page == &(Page::MainMenu { tab: *selector }){
                    style = style.fg(Color::LightYellow);
                }
                let text = match selector {
                    MainMenuSelector::Trending => "Trending",
                    MainMenuSelector::Popular => "Popular",
                    MainMenuSelector::History => "History",
                };

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(style);
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, rect);
            }
            MainMenuItem::VideoList(data) => {
                let split = HorizontalSplit::default()
                    .percentages(vec![40, 60])
                    .border_style(Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));

                let chunks = split.inner(rect);

                frame.render_widget(split, rect);

                if let Some((videos, list, _)) = data {
                    list.area(chunks[0]);
                    let mut list = list.clone();

                    if selected {
                        list.selected_style(Style::default().fg(Color::LightRed));
                    } else {
                        list.selected_style(Style::default().fg(Color::LightYellow));
                    }
                    frame.render_widget(list, chunks[0]);
                }
            }
        }
    }
}
