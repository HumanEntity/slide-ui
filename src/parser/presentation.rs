use crate::parser::Slide;

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
