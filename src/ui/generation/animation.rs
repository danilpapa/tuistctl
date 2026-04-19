use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crossterm::event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use crate::TerminalCFG;

const MATRIX_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F',
    '!', '@', '#', '$', '%', '&', '*', '+', '=', '?', '/', '\\', '|', '<', '>',
];

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

struct MatrixColumn {
    head: f32,
    speed: f32,
    length: usize,
    char_seed: usize,
    active: bool,
    reset_at: f32,
}

struct MatrixState {
    columns: Vec<MatrixColumn>,
    height: usize,
}

impl MatrixState {
    fn new(width: usize, height: usize) -> Self {
        let columns = (0..width)
            .map(|i| {
                let seed = i.wrapping_mul(2654435761).wrapping_add(1234567);
                let speed = 0.15 + (seed % 9) as f32 * 0.08;
                let length = 5 + seed % 14;
                let start = -((seed % (height + length)) as f32);
                
                MatrixColumn {
                    head: start,
                    speed,
                    length,
                    char_seed: seed.wrapping_mul(6364136223846793005),
                    active: seed % 4 != 0,
                    reset_at: height as f32 + length as f32,
                }
            })
            .collect();
        MatrixState { columns, height }
    }

    fn update(&mut self) {
        let height = self.height;
        for (i, col) in self.columns.iter_mut().enumerate() {
            if !col.active {
                continue;
            }
            col.head += col.speed;
            if col.head > col.reset_at {
                col.head = -((i.wrapping_mul(31) % (col.length + 1)) as f32);
                col.char_seed = col.char_seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                col.reset_at = height as f32 + col.length as f32;
            }
        }
    }
}

pub fn run_generation_animation(terminal: &mut TerminalCFG, cmd: &str) -> anyhow::Result<()> {
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = Arc::clone(&done);

    let workspace_dir = crate::service::file_finder::find_workspace()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let cmd = cmd.to_string();
    std::thread::spawn(move || {
        let _ = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .current_dir(&workspace_dir)
            .spawn()
            .and_then(|mut child| child.wait());
        done_clone.store(true, Ordering::Relaxed);
    });

    let size = terminal.size()?;
    let mut state = MatrixState::new(size.width as usize, size.height as usize);
    let mut frame = 0usize;

    while !done.load(Ordering::Relaxed) {
        render(terminal, &state, frame);
        if event::poll(Duration::from_millis(80))? {
            event::read()?;
        }
        state.update();
        frame = frame.wrapping_add(1);
    }

    Ok(())
}

fn render(terminal: &mut TerminalCFG, state: &MatrixState, frame: usize) {
    let spinner = SPINNER[frame % SPINNER.len()];
    let dots = match (frame / 2) % 4 {
        0 => "   ",
        1 => ".  ",
        2 => ".. ",
        _ => "...",
    };

    let cols: Vec<(i32, usize, usize, bool)> = state
        .columns
        .iter()
        .map(|c| (c.head as i32, c.length, c.char_seed, c.active))
        .collect();

    _ = terminal.draw(move |f| {
        let area = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(area);

        let matrix_h = chunks[1].height as usize;
        let matrix_w = chunks[1].width as usize;

        // --- Matrix rain ---
        let mut lines: Vec<Line> = Vec::with_capacity(matrix_h);
        for row in 0..matrix_h {
            let mut spans: Vec<Span> = Vec::with_capacity(matrix_w);
            for col_idx in 0..matrix_w.min(cols.len()) {
                let (head, length, char_seed, active) = cols[col_idx];
                if !active {
                    spans.push(Span::raw(" "));
                    continue;
                }
                let dist = head - row as i32;
                let (ch, color) = if dist == 0 {
                    let idx = (char_seed.wrapping_add(row.wrapping_mul(17))) % MATRIX_CHARS.len();
                    (MATRIX_CHARS[idx], Color::White)
                } else if dist > 0 && dist < length as i32 {
                    let fade = dist as f32 / length as f32;
                    let color = if fade < 0.25 {
                        Color::LightGreen
                    } else if fade < 0.6 {
                        Color::Green
                    } else {
                        Color::DarkGray
                    };
                    let idx = (char_seed.wrapping_add(row.wrapping_mul(31).wrapping_add(7))) % MATRIX_CHARS.len();
                    (MATRIX_CHARS[idx], color)
                } else {
                    (' ', Color::Reset)
                };
                spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
            lines.push(Line::from(spans));
        }
        f.render_widget(Paragraph::new(lines), chunks[1]);

        // --- Header (rendered after matrix so it overlays) ---
        let header = Paragraph::new(Line::from(vec![
            Span::styled("danilpapa", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("  //  ", Style::default().fg(Color::DarkGray)),
            Span::styled("generation in progress", Style::default().fg(Color::White)),
        ]))
        .alignment(Alignment::Left);
        f.render_widget(header, chunks[0]);

        // --- Footer spinner ---
        let footer = Line::from(vec![
            Span::styled(format!("  {} ", spinner), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("generating", Style::default().fg(Color::White)),
            Span::styled(dots, Style::default().fg(Color::DarkGray)),
        ]);
        f.render_widget(Paragraph::new(footer).alignment(Alignment::Left), chunks[2]);
    });
}
