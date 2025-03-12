use std::{fs::File, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, List, ListState, Padding, Paragraph, StatefulWidget, Widget},
};

const DEFAULT_ARISE_ATTEMPTS: i32 = 3;
const TITLE: &'static str = " Arise ";

use crate::models::player::Player;

#[derive(PartialEq, Eq)]
enum Mode {
    Status,
    Shadows,
    Main,
}

impl Mode {
    fn next(&self) -> Mode {
        match self {
            Mode::Status => Self::Shadows,
            Mode::Shadows => Self::Main,
            Mode::Main => Self::Status,
        }
    }
    fn prev(&self) -> Mode {
        match self {
            Mode::Status => Self::Main,
            Mode::Shadows => Self::Status,
            Mode::Main => Self::Shadows,
        }
    }
}

#[derive(PartialEq, Eq)]
enum MainMode {
    Start,
    Encounter,
    Battle,
}

pub struct Game<'a> {
    player: Player,
    file_path: &'a str,
    shadow_widget: ShadowsWidget,
    mode: Mode,
    exit: bool,
}

impl<'a> Game<'a> {
    pub fn new(file_path: &'a str) -> Game<'a> {
        Game {
            player: Player::new(None),
            file_path,
            shadow_widget: ShadowsWidget::new(vec![]),
            mode: Mode::Main,
            exit: false,
        }
    }

    pub fn load(file_path: &'a str) -> io::Result<Game<'a>> {
        let file = File::open(file_path)?;
        let player: Player = serde_json::from_reader(file)?;
        let shadow_list = player
            .shadows
            .iter()
            .map(|s| format!("{} - Rank: {}, Power: {}", s.name, s.rank, s.power))
            .collect();

        Ok(Game {
            player,
            file_path,
            shadow_widget: ShadowsWidget::new(shadow_list),
            mode: Mode::Main,
            exit: false,
        })
    }

    pub fn save_state(&self) -> io::Result<()> {
        let file = File::create(self.file_path)?;
        serde_json::to_writer_pretty(file, &self.player)?;
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press {
                    if self.update_key(&event) {
                        return Ok(());
                    }
                    match self.mode {
                        Mode::Main => self.update_key_main(&event),
                        _ => {}
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn update_key_main(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Enter => {
                // Here
            }
            _ => {}
        }
    }

    fn update_key(&mut self, event: &KeyEvent) -> bool {
        match event.code {
            KeyCode::Char('q') => {
                self.exit = true;
                true
            }
            KeyCode::Tab => {
                self.mode = self.mode.next();
                true
            }
            KeyCode::BackTab => {
                self.mode = self.mode.prev();
                true
            }
            _ => false,
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [top, down] =
            Layout::vertical(Constraint::from_percentages([40, 60])).areas(frame.area());

        let [left, right] = Layout::horizontal(Constraint::from_percentages([40, 60])).areas(top);

        frame.render_widget(self.status_widget(), left);

        self.shadow_widget
            .draw(frame, right, self.mode == Mode::Shadows);

        frame.render_widget(self, down);
    }

    fn status_widget(&self) -> Paragraph<'_> {
        let title = Line::from(" Status ".bold());
        let text = vec![
            Line::from(vec!["Name: ".bold(), self.player.name.clone().into()]),
            Line::from(vec![
                "Health: ".bold(),
                self.player.health.to_string().into(),
            ]),
        ];

        Paragraph::new(text).white().block(
            border_block(self.mode == Mode::Status)
                .title(title.centered())
                .padding(Padding::horizontal(1)),
        )
    }
}

impl<'a> Widget for &mut Game<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(TITLE.bold().magenta());

        let instructions = Line::from(vec![
            " Continue ".white(),
            "<Enter> ".blue().bold(),
            "Quit ".white(),
            "<Q> ".blue().bold(),
        ]);

        let text = vec![
            Line::from(vec!["hello world".into()]),
            Line::from(vec!["hello world".into()]),
        ];
        Paragraph::new(text)
            .white()
            .alignment(Alignment::Center)
            .block(
                border_block(self.mode == Mode::Main)
                    .title(title.left_aligned())
                    .title_bottom(instructions.centered())
                    .padding(Padding::horizontal(1)),
            )
            .render(area, buf);
    }
}

struct ShadowsWidget {
    state: ListState,
    list: Vec<String>,
}

impl ShadowsWidget {
    fn new(shadow_list: Vec<String>) -> ShadowsWidget {
        ShadowsWidget {
            state: ListState::default(),
            list: shadow_list,
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let title = Line::from(" Shadows ".bold());
        let list = List::new(self.list.clone()).white().block(
            border_block(focused)
                .title(title.centered())
                .padding(Padding::horizontal(1)),
        );
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}

fn border_block(selected: bool) -> Block<'static> {
    let block = Block::bordered().border_set(border::ROUNDED);
    match selected {
        true => block.light_magenta(),
        false => block,
    }
}
