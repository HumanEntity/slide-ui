use crossterm::cursor::{Hide, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::{queue, Result};

use std::io::{stdout, Write};
use std::time::Duration;

use crate::event::{Event as Event2, EventSystem};
use crate::parser::presentation::{Hunk, Presentation};
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Renderer {
    pub content: Presentation,
    pub size: (u16, u16),
    pub closed: bool,
    pres_pos: PresentationPosition,
    config: toml::Table,
}

impl Renderer {
    // Constructor
    #[inline]
    pub fn new(content: Presentation, config: toml::Table) -> Result<Self> {
        let x = Self {
            content,
            size: size()?,
            closed: false,
            pres_pos: PresentationPosition {
                slide: 0,
                scroll: 0,
            },
            config,
        };
        x.enable_raw_mode()?;
        Ok(x)
    }
    // Processing
    #[must_use]
    pub const fn is_running(&self) -> bool {
        !self.closed
    }
    pub fn process(&mut self) -> Result<()> {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(event) => Self::check_key(event),
                Event::Resize(x, y) => self.size = (x, y),
                _ => {}
            }
        }

        if let Some(event) = EventSystem::pop() {
            match event {
                Event2::Closed => self.closed = true,
                Event2::PrevSlide => {
                    if self.pres_pos.slide != 0 {
                        self.pres_pos.slide -= 1;
                    }
                }
                Event2::NextSlide => {
                    if (self.pres_pos.slide as usize) < self.content.slides.len() - 1 {
                        self.pres_pos.slide += 1;
                    }
                }
                _ => {}
            }
        }

        self.draw()?;

        Ok(())
    }
    fn check_key(event: KeyEvent) {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => EventSystem::push(Event2::Closed),
            KeyCode::Up => EventSystem::push(Event2::ScrollUp),
            KeyCode::Down => EventSystem::push(Event2::ScrollDown),
            KeyCode::Left => EventSystem::push(Event2::PrevSlide),
            KeyCode::Right => EventSystem::push(Event2::NextSlide),
            _ => {}
        }
    }
    // Rendering
    fn draw(&self) -> Result<()> {
        Self::clear()?;
        //
        // for (i, hunk) in self.content.slides[self.pres_pos.slide as usize]
        //     .content
        //     .iter()
        //     .enumerate()
        // {
        //     Self::draw_hunk(hunk, self.pres_pos.scroll + (i as u16), 0)?;
        // }

        self.content.slides[self.pres_pos.slide as usize]
            .content
            .iter()
            .enumerate()
            .map(|(i, hunk)| Self::draw_hunk(hunk, self.pres_pos.scroll + (i as u16), 0))
            .for_each(drop);

        self.draw_slide_indicator()?;

        stdout().flush()?;
        Ok(())
    }
    fn clear() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))
    }
    fn draw_hunk(hunk: &Hunk, row: u16, col: u16) -> Result<()> {
        queue!(
            stdout(),
            crossterm::cursor::MoveTo(col, row),
            SetForegroundColor(hunk.fg_color),
            SetBackgroundColor(hunk.bg_color),
            Print(hunk.content.clone())
        )?;
        Ok(())
    }
    fn draw_slide_indicator(&self) -> Result<()> {
        macro_rules! get {
            ($map:expr, $value:ident, $key:expr, $code:block, $else:block) => {
                if let Some(toml::Value::Table($value)) = $map.get($key) {
                    $code
                } else {
                    $else
                }
            };
        }
        let slide_indicator = format!("{}/{}", self.content.slides.len(), self.pres_pos.slide + 1);
        let column = self.size.0 - 5 - (slide_indicator.len() as u16);
        let row = self.size.1 - 5;
        let (bg_color, fg_color) = get!(
            self.config,
            display,
            "display",
            {
                get!(
                    display,
                    slide_indicator,
                    "slide_indicator",
                    {
                        let bg = Parser::match_color(
                            slide_indicator
                                .get("bg")
                                .unwrap_or(&toml::Value::String(String::from("black")))
                                .clone()
                                .as_str()
                                .unwrap_or("black"),
                        );

                        let fg = Parser::match_color(
                            slide_indicator
                                .get("fg")
                                .unwrap_or(&toml::Value::String(String::from("white")))
                                .clone()
                                .as_str()
                                .unwrap_or(""),
                        );
                        (bg, fg)
                    },
                    { (Color::Black, Color::White) }
                )
            },
            { (Color::Black, Color::White) }
        );
        queue!(
            stdout(),
            crossterm::cursor::MoveTo(column, row),
            SetForegroundColor(fg_color),
            SetBackgroundColor(bg_color),
            Print(slide_indicator),
        )?;
        Ok(())
    }
    // Raw Mode Managment
    pub fn enable_raw_mode(&self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout(), Hide)
    }
    pub fn disable_raw_mode(&self) -> Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        execute!(stdout(), Show)
    }
}

#[derive(Debug, Clone)]
pub struct PresentationPosition {
    pub slide: u16,
    pub scroll: u16,
}
