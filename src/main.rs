use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: i8,
    exit: bool,
}

impl App {
    /// runs app's main loop until user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // checking key **press** because crossterm emits key release and repeat too (on W*ndows)
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.dec_counter(),
            KeyCode::Right => self.inc_counter(),
            _ => {}
        }
    }
    // todo some test stuff from the example page, might be cool to look at
    // https://ratatui.rs/tutorials/counter-app/basic-app/

    fn exit(&mut self) {
        self.exit = true;
    }

    fn inc_counter(&mut self) {
        self.counter += 1;
    }

    fn dec_counter(&mut self) {
        self.counter -= 1;
    }
}

// render_widget from Frame is important, renders any type that impl's Widget trait
// thus we impl it for a ref to App (ref because we don't mutate state)
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // added alignment() below to illustrate you can add it anywhere
        // assuming if you want to use same line with diff alignment, you could call alignment
        // with something else every time you want to use it?
        let title = Line::from("Counter App Tutorial".bold()).alignment(Alignment::Center);
        let instr = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            // .title(title.alignment(Alignment::Center))
            .title_top(title)
            // here alignment is set only at this point, unlike above example
            .title_bottom(instr.alignment(Alignment::Center))
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
