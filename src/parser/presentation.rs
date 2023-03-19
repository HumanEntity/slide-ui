pub struct Presentation {
    pub slides: Vec<Slide>,
}

impl Presentation{
    pub fn new(slides: Vec<Slide>) -> Self{
        Self {
            slides,
        }
    }
    pub fn get(&self, id: usize) -> Slide {
        self.slides.get(id).unwrap().clone()
    }
}

#[derive(Debug, Clone)]
pub struct Slide{
    content: Vec<Hunk>,
}

#[derive(Debug, Clone)]
pub struct Hunk{
    content: String,
    color: crossterm::style::Color,
}
