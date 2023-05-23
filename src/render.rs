use crossterm::cursor::{Hide, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::style::{Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::{queue, Result};

use std::io::{stdout, Write};
use std::time::Duration;

use crate::event::{Event as Event2, EventSystem};
use crate::parser::presentation::{Hunk, Presentation};

#[derive(Debug, Clone)]
pub struct Renderer {
    pub content: Presentation,
    pub size: (u16, u16),
    pub closed: bool,
    pres_pos: PresentationPosition,
}

impl Renderer {
    // Constructor
    #[inline]
    pub fn new(content: Presentation) -> Result<Self> {
        let x = Self {
            content,
            size: size()?,
            closed: false,
            pres_pos: PresentationPosition {
                slide: 0,
                scroll: 0,
            },
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

        for (i, hunk) in self.content.slides[self.pres_pos.slide as usize]
            .content
            .iter()
            .enumerate()
        {
            Self::draw_hunk(hunk, self.pres_pos.scroll + (i as u16), 0)?;
        }

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
