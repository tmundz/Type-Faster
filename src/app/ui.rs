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
    // Vertical layout
    let padding_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(30),
            ]
            .as_ref(),
        )
        .split(padding_chunk[1]);

    let chunks2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(50), Constraint::Percentage(66)].as_ref())
        .split(main_chunk[0]);
    //rect.render_widget(title, chunks[0]);
    draw_main(rect, main_chunk[1]);
    draw_upper(rect, chunks2[1]);
    draw_lower(rect, chunks2[0]);
}

fn draw_main<B: Backend>(rect: &mut Frame<B>, chunk: Rect) {
    let widget = Paragraph::new("Plop with TUI")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    rect.render_widget(widget, chunk)
}

fn draw_upper<B: Backend>(rect: &mut Frame<B>, chunk: Rect) {
    let widget = Paragraph::new("Upper")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    rect.render_widget(widget, chunk)
}

fn draw_lower<B: Backend>(rect: &mut Frame<B>, chunk: Rect) {
    let padding_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Percentage(66)])
        .split(chunk);

    let widget = Paragraph::new("Lower")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    rect.render_widget(widget, padding_chunks[0])
}
