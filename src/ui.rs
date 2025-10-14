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
    // Calculate exact board height (19 lines for 19x19 board)
    let board_height = game.board.size as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),                // Header
            Constraint::Length(board_height + 2), // Board + padding
            Constraint::Min(0),                   // Fill remaining space
            Constraint::Length(3),                // Status
        ])
        .split(frame.area());

    render_header(frame, chunks[0], game);
    render_board(frame, chunks[1], &game.board);
    render_status(frame, chunks[3], game, auto_play);
}

fn render_header(frame: &mut Frame, area: Rect, game: &GameState) {
    let black_player = game.get_property("PB").unwrap_or("Black");
    let white_player = game.get_property("PW").unwrap_or("White");
    let game_name = game.get_property("GN").unwrap_or("Go Game");

    // Create moving shine effect for game title with 3-character gradient
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    // Pale orange colors with more distinct differences for 3-char gradient
    let base_color = RatatuiColor::Rgb(255, 190, 140); // Darker base
    let side_bright_color = RatatuiColor::Rgb(255, 210, 165); // Side glow
    let center_color = RatatuiColor::Rgb(255, 240, 210); // Very bright center

    let mut title_spans = Vec::new();
    let chars: Vec<char> = game_name.chars().collect();
    let speed = 150; // milliseconds per character

    // Bidirectional movement: right-to-left, pause, left-to-right, pause
    let cycle_length = (chars.len() as u128 + 4) * 2 + 8; // +4 for pause at each end
    let position_in_cycle = (now / speed) % cycle_length;

    let shine_center = if position_in_cycle < chars.len() as u128 + 4 {
        // Moving right-to-left
        chars.len() as i128 - position_in_cycle as i128
    } else {
        // Moving left-to-right (after pause)
        position_in_cycle as i128 - (chars.len() as i128 + 8)
    };

    for (i, ch) in chars.iter().enumerate() {
        let distance_from_center = i as i128 - shine_center;

        // Create 3-character gradient with clear distinction
        let style = if distance_from_center == 0 {
            // Center: brightest with bold
            Style::default()
                .fg(center_color)
                .add_modifier(Modifier::BOLD)
        } else if distance_from_center == -1 || distance_from_center == 1 {
            // Adjacent: medium bright (no bold)
            Style::default().fg(side_bright_color)
        } else {
            // Base color for everything else
            Style::default().fg(base_color)
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

    // Add padding around the board to create a "board on table" effect
    // Limit board dimensions, center it if window is larger
    use ratatui::layout::Margin;
    let max_board_width = 65;
    let max_board_height = 27;

    let horizontal_padding = if area.width > max_board_width {
        (area.width - max_board_width) / 2
    } else {
        2 // Minimum padding
    };

    let vertical_padding = if area.height > max_board_height {
        (area.height - max_board_height) / 2
    } else {
        1 // Minimum padding
    };

    let board_area = area.inner(Margin {
        horizontal: horizontal_padding,
        vertical: vertical_padding,
    });

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default().style(Style::default().bg(RatatuiColor::Rgb(210, 180, 140))), // Tan wood color
    );

    frame.render_widget(paragraph, board_area);
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

    // Play status with emoji
    spans.push(Span::raw(" "));
    if auto_play {
        spans.push(Span::styled(
            "▶️", // Play button emoji
            Style::default().fg(RatatuiColor::Green),
        ));
    } else {
        spans.push(Span::styled(
            "⏸️", // Pause button emoji
            Style::default().fg(RatatuiColor::Yellow),
        ));
    }

    // Loop status with emoji
    spans.push(Span::raw(" "));
    if game.is_looping_enabled() {
        spans.push(Span::styled(
            "🔁", // Repeat/loop emoji
            Style::default().fg(RatatuiColor::Magenta),
        ));
    } else {
        spans.push(Span::styled(
            "➡️", // Right arrow emoji (no loop, just forward)
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
