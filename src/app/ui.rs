/*
 * This view will control the tui
 * When a word gets completed then this will send the word as completed
 *
 */
//use super::{Model, RoundState};
use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn render<B: Backend>(rect: &mut Frame<B>) {
    let size = rect.size();
    let title = draw_title();
    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    // Title

    rect.render_widget(title, chunks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    return Paragraph::new("Plop with TUI")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
}
