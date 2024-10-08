use ratatui::widgets::{Block, Widget};

pub const MENU_HEIGHT: u16 = 4;

#[derive(Debug)]
pub struct LeaderMenu {}

impl LeaderMenu {
    pub fn new() -> Self {
        return LeaderMenu {};
    }
}

impl Widget for LeaderMenu {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Block::new().render(area, buf);
    }
}
