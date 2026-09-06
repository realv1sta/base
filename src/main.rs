use std::io::{self, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum Mode {
    Normal,
    Insert,
    Command,
}

struct Buffer {
    filename: String,
    lines: Vec<String>,
    cy: usize,
    cx: usize,
}

impl Buffer {
    fn new(filename: String) -> Self {
        let lines = match std::fs::read_to_string(&filename) {
            Ok(content) => {
                if content.is_empty() {
                    vec![String::new()]
                } else {
                    content.lines().map(|l| l.to_string()).collect()
                }
            }
            Err(_) => vec![String::new()],
        };
        Self {
            filename,
            lines,
            cy: 0,
            cx: 0,
        }
    }

    fn current_line(&self) -> &str {
        &self.lines[self.cy.min(self.lines.len() - 1)]
    }

    fn clamp_cursor(&mut self) {
        let max_line = self.lines.len().saturating_sub(1);
        self.cy = self.cy.min(max_line);
        let col = self.lines[self.cy].chars().count();
        self.cx = self.cx.min(col);
    }

    fn move_cursor_up(&mut self) {
        if self.cy > 0 {
            self.cy -= 1;
            self.clamp_cursor();
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cy + 1 < self.lines.len() {
            self.cy += 1;
            self.clamp_cursor();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cx > 0 {
            self.cx -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        let col = self.current_line().chars().count();
        if self.cx < col {
            self.cx += 1;
        }
    }

    fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cy];
        let mut bytes: Vec<char> = line.chars().collect();
        bytes.insert(self.cx, c);
        *line = bytes.into_iter().collect();
        self.cx += 1;
    }

    fn backspace(&mut self) {
        if self.cx > 0 {
            let line = &mut self.lines[self.cy];
            let mut bytes: Vec<char> = line.chars().collect();
            bytes.remove(self.cx - 1);
            *line = bytes.into_iter().collect();
            self.cx -= 1;
        } else if self.cy > 0 {
            let prev_len = self.lines[self.cy - 1].chars().count();
            let cur = self.lines.remove(self.cy);
            self.lines[self.cy - 1].push_str(&cur);
            self.cy -= 1;
            self.cx = prev_len;
        }
    }

    fn newline(&mut self) {
        let line = &self.lines[self.cy];
        let split = line.char_indices().nth(self.cx).map(|(i, _)| i).unwrap_or(line.len());
        let rest = line[split..].to_string();
        self.lines[self.cy].truncate(split);
        self.lines.insert(self.cy + 1, rest);
        self.cy += 1;
        self.cx = 0;
    }

    fn save(&self) -> io::Result<()> {
        let content = self.lines.join("\n") + "\n";
        std::fs::write(&self.filename, content)
    }
}

fn main() -> Result<()> {
    let filename = std::env::args().nth(1).unwrap_or_else(|| "untitled.txt".to_string());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    queue!(stdout, EnterAlternateScreen, Hide)?;
    stdout.flush()?;

    let mut buffer = Buffer::new(filename);
    let mut mode = Mode::Normal;
    let mut running = true;

    let run = (|| {
        while running {
            render(&mut stdout, &buffer, &mode)?;
            if let Event::Key(key) = event::read()? {
                handle_key(key, &mut buffer, &mut mode, &mut running)?;
            }
        }
        Ok(())
    })();

    queue!(stdout, Show, LeaveAlternateScreen)?;
    stdout.flush()?;
    disable_raw_mode()?;
    run
}

fn handle_key(
    key: KeyEvent,
    buffer: &mut Buffer,
    mode: &mut Mode,
    running: &mut bool,
) -> Result<()> {
    match mode {
        Mode::Normal => match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                *running = false;
            }
            KeyCode::Char('h') => buffer.move_cursor_left(),
            KeyCode::Char('j') => buffer.move_cursor_down(),
            KeyCode::Char('k') => buffer.move_cursor_up(),
            KeyCode::Char('l') => buffer.move_cursor_right(),
            KeyCode::Char('i') => *mode = Mode::Insert,
            KeyCode::Char('a') => {
                buffer.move_cursor_right();
                *mode = Mode::Insert;
            }
            KeyCode::Char('A') => {
                buffer.cx = buffer.current_line().chars().count();
                *mode = Mode::Insert;
            }
            KeyCode::Char(':') => *mode = Mode::Command,
            KeyCode::Char('x') => {
                let line = &mut buffer.lines[buffer.cy];
                if buffer.cx < line.chars().count() {
                    let mut bytes: Vec<char> = line.chars().collect();
                    bytes.remove(buffer.cx);
                    *line = bytes.into_iter().collect();
                }
            }
            KeyCode::Char('w') => {
                let mut out = String::new();
                out.push_str(&format!(
                    "{{\"event\": \"key\", \"mode\": \"NORMAL\", \"cy\": {}, \"cx\": {}}}",
                    buffer.cy + 1,
                    buffer.cx + 1
                ));
                println!("{}", out);
            }
            _ => {}
        },
        Mode::Insert => match key.code {
            KeyCode::Esc => *mode = Mode::Normal,
            KeyCode::Char(c) => buffer.insert_char(c),
            KeyCode::Backspace => buffer.backspace(),
            KeyCode::Enter => buffer.newline(),
            _ => {}
        },
        Mode::Command => match key.code {
            KeyCode::Esc => *mode = Mode::Normal,
            KeyCode::Enter => {
                *mode = Mode::Normal;
            }
            _ => {}
        },
    }
    Ok(())
}

fn render(stdout: &mut io::Stdout, buffer: &Buffer, mode: &Mode) -> Result<()> {
    queue!(stdout, Clear(ClearType::All))?;

    let (width, height) = crossterm::terminal::size()?;

    let status = match mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Command => "COMMAND",
    };

    let status_text = format!(
        " Base | {} | MODE: {} | Line: {}/{} Col: {} ",
        buffer.filename,
        status,
        buffer.cy + 1,
        buffer.lines.len(),
        buffer.cx + 1
    );

    queue!(
        stdout,
        SetBackgroundColor(Color::White),
        SetForegroundColor(Color::Black),
        MoveTo(0, 0),
        Print(format!("{:<width$}", status_text, width = width as usize)),
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset),
    )?;

    let mut row = 1usize;
    let mut cursor_row = 1usize;
    for (idx, line) in buffer.lines.iter().enumerate() {
        if row >= height as usize {
            break;
        }
        let visible = &line[..line.len().min(width as usize - 1)];
        queue!(stdout, MoveTo(0, row as u16), Print(visible))?;
        if idx == buffer.cy {
            cursor_row = row;
        }
        row += 1;
    }

    let cursor_x = buffer.cx as u16;
    let cursor_y = std::cmp::min(cursor_row as u16, height.saturating_sub(2));
    queue!(stdout, MoveTo(cursor_x, cursor_y))?;
    stdout.flush()?;
    Ok(())
}
