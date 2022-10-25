
/* 
 * This view will control the tui
 * When a word gets completed then this will send the word as completed
 *
 */

use std::(collections::VecDeque, iter::FromIterator);

use super::{Model, RoundState};
use tui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Layout, Direction, Corner, Rect, Alignment},
    style::{Modifier, Style, Color},
    text::{Spans, Span},
    widgets::{Block, List, ListItem, Borders, Paragraph, Wrap}
};


pub fn render<B: Backend>(f: &mut Frame<B>, model: &Model) {
    let padded_chunks = Layout::default()
        .direction(Direction::Horizantal)
        .constraints([Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25)]).as_ref()
        .split(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0).as_ref())
        .split(Padding_chunks[1]);

        ])
}
