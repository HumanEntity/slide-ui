#[derive(Debug, Clone)]
pub struct Presentation {
    pub slides: Vec<Slide>,
}

impl Presentation {
    #[must_use]
    pub fn new(slides: Vec<Slide>) -> Self {
        Self { slides }
    }
    #[must_use]
    pub fn get(&self, id: usize) -> Option<Slide> {
        self.slides.get(id).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Slide {
    pub content: Vec<Hunk>,
}

#[macro_export]
macro_rules! new_slide {
    () => {
        Slide {
            content: Vec::new(),
        }
    };
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub content: String,
    pub bg_color: crossterm::style::Color,
    pub fg_color: crossterm::style::Color,
}
