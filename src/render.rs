use crossterm::cursor::{Hide, Show};
use crossterm::{Result, queue};
use crossterm::event::{Event, KeyEvent, KeyCode, read, poll};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::execute;

use std::time::Duration;
use std::io::{stdout, Write};

use crate::event::{EventSystem, BaseEvent};
use crate::parser::presentation::{Presentation, Hunk};

#[derive(Debug, Clone)]
pub struct Renderer{
    pub content: Presentation,
    pub size: (u16, u16),
    pub closed: bool,
    pres_pos: PresentationPosition,
}

impl Renderer{
    // Constructor
    #[inline]
    pub fn new(content: Presentation) -> Result<Self> {
        let x = Self {
            content,
            size: size()?,
            closed: false,
            pres_pos: PresentationPosition { slide: 0, scroll: 0 }
        };
        x.enable_raw_mode()?;
        Ok(x)
    }
    // Processing
    pub fn is_running(&self) -> bool { !self.closed }
    pub fn process(&mut self) -> Result<()> {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(event) => self.check_key(event)?,
                Event::Resize(x, y) => self.size = (x, y),
                _ => {}
            }
        }

        if let Some(event) = EventSystem::pop() {
            match event.content.as_str(){
                "Closed" => self.closed = true,
                "PrevSlide" => {
                    if self.pres_pos.slide != 0 {
                        self.pres_pos.slide -= 1;
                    }
                }
                "NextSlide" => {
                    if (self.pres_pos.slide as usize) < self.content.slides.len() - 1 {
                        self.pres_pos.slide += 1;
                    }
                },
                _ => {}
            }
        }

        self.draw()?;

        Ok(())
    }
    fn check_key(&self, event: KeyEvent) -> Result<()> {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => EventSystem::push(BaseEvent::Closed.into()),
            KeyCode::Up => EventSystem::push(BaseEvent::ScrollUp.into()),
            KeyCode::Down => EventSystem::push(BaseEvent::ScrollDown.into()),
            KeyCode::Left => EventSystem::push(BaseEvent::PrevSlide.into()),
            KeyCode::Right => EventSystem::push(BaseEvent::NextSlide.into()),
            _ => {}
        }
        Ok(())
    }
    // Rendering
    fn draw(&self) -> Result<()> {
        self.clear()?;

        let mut i = 0;
        for hunk in &self.content.slides[self.pres_pos.slide as usize].content {
            self.draw_hunk(hunk, self.pres_pos.scroll+i, 0)?;
            i+=1;
        }

        stdout().flush()?;
        Ok(())
    }
    fn clear(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))
    }
    fn draw_hunk(&self, hunk: &Hunk, row: u16, col: u16) -> Result<()> {
        queue!(stdout(), crossterm::cursor::MoveTo(col, row), SetForegroundColor(hunk.fg_color), SetBackgroundColor(hunk.bg_color), Print(hunk.content.clone()))?;
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
