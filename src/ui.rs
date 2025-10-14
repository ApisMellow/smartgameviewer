use crate::game::{Board, GameState};
use crate::parser::Color;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as RatatuiColor, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_game(frame: &mut Frame, game: &GameState, auto_play: bool) {
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
    render_status(frame, chunks[2], game, auto_play);
}

fn render_header(frame: &mut Frame, area: Rect, game: &GameState) {
    let black_player = game.get_property("PB").unwrap_or("Black");
    let white_player = game.get_property("PW").unwrap_or("White");
    let game_name = game.get_property("GN").unwrap_or("Go Game");

    let text = Line::from(vec![
        Span::raw(format!("{} | ", game_name)),
        Span::styled(black_player, Style::default().fg(RatatuiColor::White)),
        Span::raw(" vs "),
        Span::styled(white_player, Style::default().fg(RatatuiColor::Gray)),
    ]);

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

fn render_status(frame: &mut Frame, area: Rect, game: &GameState, auto_play: bool) {
    let play_status = if auto_play { "[PLAYING]" } else { "[PAUSED]" };
    let loop_status = if game.is_looping_enabled() {
        "[LOOP]"
    } else {
        "[NO LOOP]"
    };

    let current_move_info = if game.current_move > 0 && game.current_move <= game.moves.len() {
        let mv = &game.moves[game.current_move - 1];
        let color = match mv.color {
            Color::Black => "Black",
            Color::White => "White",
        };
        let pos = if let Some((row, col)) = mv.position {
            format!("{}{}", (b'A' + col) as char, row + 1)
        } else {
            "Pass".to_string()
        };
        format!(" | {} {}", color, pos)
    } else {
        "".to_string()
    };

    let move_info = format!(
        "Move {}/{}{} {} {} | ← → Step | Space Play/Pause | L Loop | Q Quit",
        game.current_move,
        game.moves.len(),
        current_move_info,
        play_status,
        loop_status
    );

    let paragraph = Paragraph::new(move_info)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
