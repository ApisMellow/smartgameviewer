use crate::game::{Board, GameState};
use crate::parser::Color;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_game(frame: &mut Frame, game: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Board
            Constraint::Length(3), // Status
        ])
        .split(frame.area());

    render_header(frame, chunks[0], game);
    render_board(frame, chunks[1], &game.board);
    render_status(frame, chunks[2], game);
}

fn render_header(frame: &mut Frame, area: Rect, _game: &GameState) {
    let text = Line::from(vec![Span::raw("Smart Game Viewer")]);

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(paragraph, area);
}

fn render_board(frame: &mut Frame, area: Rect, board: &Board) {
    let size = board.size as usize;
    let mut lines = Vec::new();

    for row in 0..size {
        let mut line_content = String::new();

        for col in 0..size {
            let stone = match board.get(row as u8, col as u8) {
                Some(Color::Black) => "●",
                Some(Color::White) => "○",
                None => {
                    // Draw intersection
                    if row == 0 && col == 0 {
                        "┌"
                    } else if row == 0 && col == size - 1 {
                        "┐"
                    } else if row == size - 1 && col == 0 {
                        "└"
                    } else if row == size - 1 && col == size - 1 {
                        "┘"
                    } else if row == 0 {
                        "┬"
                    } else if row == size - 1 {
                        "┴"
                    } else if col == 0 {
                        "├"
                    } else if col == size - 1 {
                        "┤"
                    } else {
                        "┼"
                    }
                }
            };

            line_content.push_str(stone);

            // Add horizontal line between intersections (except last column)
            if col < size - 1 {
                line_content.push('─');
            }
        }

        lines.push(Line::from(line_content));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_status(frame: &mut Frame, area: Rect, game: &GameState) {
    let move_info = format!(
        "Move {}/{} | ← → Step | Space Play/Pause | Q Quit",
        game.current_move,
        game.moves.len()
    );

    let paragraph = Paragraph::new(move_info)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
