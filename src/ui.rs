use crate::game::{Board, GameState};
use crate::parser::Color;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as RatatuiColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{SystemTime, UNIX_EPOCH};

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

    // Create moving shine effect for game title
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    // Pale orange color (similar to Claude Code thinking line)
    let base_color = RatatuiColor::Rgb(255, 200, 150);
    let bright_color = RatatuiColor::Rgb(255, 220, 180);

    let mut title_spans = Vec::new();
    let chars: Vec<char> = game_name.chars().collect();
    let speed = 200; // milliseconds per character

    for (i, ch) in chars.iter().enumerate() {
        // Calculate phase for this character based on time and position
        let phase = ((now / speed) + i as u128) % (chars.len() as u128 * 2);

        // Create shine wave effect
        let color = if phase == i as u128 || phase == (chars.len() as u128 * 2 - 1 - i as u128) {
            bright_color
        } else {
            base_color
        };

        let style = if phase == i as u128 || phase == (chars.len() as u128 * 2 - 1 - i as u128) {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        title_spans.push(Span::styled(ch.to_string(), style));
    }

    title_spans.push(Span::raw(" "));
    title_spans.push(Span::styled(
        "│ ",
        Style::default().fg(RatatuiColor::DarkGray),
    ));
    title_spans.push(Span::styled(
        black_player,
        Style::default().fg(RatatuiColor::White),
    ));
    title_spans.push(Span::styled(
        " vs ",
        Style::default().fg(RatatuiColor::DarkGray),
    ));
    title_spans.push(Span::styled(
        white_player,
        Style::default().fg(RatatuiColor::Rgb(255, 255, 255)),
    ));

    let text = Line::from(title_spans);

    let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(RatatuiColor::DarkGray)),
    );

    frame.render_widget(paragraph, area);
}

fn render_board(frame: &mut Frame, area: Rect, board: &Board) {
    let size = board.size as usize;
    let mut lines = Vec::new();

    for row in 0..size {
        let mut spans = Vec::new();

        for col in 0..size {
            match board.get(row as u8, col as u8) {
                Some(Color::Black) => {
                    // Black stone emoji - naturally takes 2 char widths
                    spans.push(Span::styled("⚫", Style::default()));
                }
                Some(Color::White) => {
                    // White stone emoji - naturally takes 2 char widths
                    spans.push(Span::styled("⚪", Style::default()));
                }
                None => {
                    // Draw intersection in dark gray (toned down)
                    // Use 2 characters for each intersection to match emoji width
                    let intersection = if row == 0 && col == 0 {
                        "┌─"
                    } else if row == 0 && col == size - 1 {
                        "─┐"
                    } else if row == size - 1 && col == 0 {
                        "└─"
                    } else if row == size - 1 && col == size - 1 {
                        "─┘"
                    } else if row == 0 {
                        "─┬"
                    } else if row == size - 1 {
                        "─┴"
                    } else if col == 0 {
                        "├─"
                    } else if col == size - 1 {
                        "─┤"
                    } else {
                        "─┼"
                    };
                    spans.push(Span::styled(
                        intersection,
                        Style::default().fg(RatatuiColor::DarkGray),
                    ));
                }
            }

            // Add horizontal line between intersections (except last column)
            if col < size - 1 {
                spans.push(Span::styled(
                    "─",
                    Style::default().fg(RatatuiColor::DarkGray),
                ));
            }
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_status(frame: &mut Frame, area: Rect, game: &GameState, auto_play: bool) {
    let mut spans = Vec::new();

    // Move counter
    spans.push(Span::styled(
        format!("Move {}/{}", game.current_move, game.moves.len()),
        Style::default().fg(RatatuiColor::Cyan),
    ));

    // Current move info
    if game.current_move > 0 && game.current_move <= game.moves.len() {
        let mv = &game.moves[game.current_move - 1];
        let (color_text, color_style) = match mv.color {
            Color::Black => ("Black", Style::default().fg(RatatuiColor::White)),
            Color::White => (
                "White",
                Style::default().fg(RatatuiColor::Rgb(255, 255, 255)),
            ),
        };
        let pos = if let Some((row, col)) = mv.position {
            format!("{}{}", (b'A' + col) as char, row + 1)
        } else {
            "Pass".to_string()
        };
        spans.push(Span::raw(" | "));
        spans.push(Span::styled(format!("{} {}", color_text, pos), color_style));
    }

    // Play status
    spans.push(Span::raw(" "));
    if auto_play {
        spans.push(Span::styled(
            "[PLAYING]",
            Style::default().fg(RatatuiColor::Green),
        ));
    } else {
        spans.push(Span::styled(
            "[PAUSED]",
            Style::default().fg(RatatuiColor::Yellow),
        ));
    }

    // Loop status
    spans.push(Span::raw(" "));
    if game.is_looping_enabled() {
        spans.push(Span::styled(
            "[LOOP]",
            Style::default().fg(RatatuiColor::Magenta),
        ));
    } else {
        spans.push(Span::styled(
            "[NO LOOP]",
            Style::default().fg(RatatuiColor::DarkGray),
        ));
    }

    // Controls
    spans.push(Span::styled(
        " | ← → Step | Space Play/Pause | L Loop | Q Quit",
        Style::default().fg(RatatuiColor::DarkGray),
    ));

    let paragraph = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(RatatuiColor::DarkGray)),
        );

    frame.render_widget(paragraph, area);
}
