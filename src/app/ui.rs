/*
 * This view will control the tui
 *
 * When a word gets completed then this will send the word as completed
 *
 */
//use super::{Model, RoundState};
use crate::get_phrase;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn render<B: Backend>(rect: &mut Frame<B>, phrase: Vec<String>) {
    let size = rect.size();
    // Vertical layout
    let padding_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
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
        .constraints([Constraint::Percentage(50), Constraint::Length(3)].as_ref())
        .split(main_chunk[1]);

    //rect.render_widget(title, chunks[0]);
    draw_phrase(rect, main_chunk[0], phrase);
    draw_tracker(rect, chunks2[1]);
    draw_input(rect, chunks2[0]);
}

fn draw_phrase<B: Backend>(rect: &mut Frame<B>, chunk: Rect, phrase: Vec<String>) {
    let testing_phrase: &str = &phrase[0];
    let widget = Paragraph::new(testing_phrase)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    rect.render_widget(widget, chunk);
}

fn draw_input<B: Backend>(rect: &mut Frame<B>, chunk: Rect) {
    let widget = Paragraph::new("Upper")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    rect.render_widget(widget, chunk);
}

fn draw_tracker<B: Backend>(rect: &mut Frame<B>, chunk: Rect) {
    let padding_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Percentage(70)])
        .split(chunk);

    let widget = Paragraph::new("Lower")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    rect.render_widget(widget, padding_chunks[0]);
}
