use std::collections::VecDeque;

static mut EVENTS: EventQueue = EventQueue::new();

pub type EventQueue = VecDeque<Event>;

#[derive(Debug, Clone)]
pub struct EventSystem;

impl EventSystem{
    pub fn push(event: Event) {
        unsafe { EVENTS.push_back(event); }
    }
    pub fn pop() -> Option<Event> {
        unsafe { EVENTS.pop_front() }
    }
}

#[derive(Debug)]
pub struct Event {
    pub content: String,
}

#[derive(Debug, Clone, Copy)]
pub enum BaseEvent{
    Closed,
    NextSlide,
    PrevSlide,
    ScrollDown,
    ScrollUp,
}

impl From<BaseEvent> for Event{
    fn from(value: BaseEvent) -> Self {
        Self {
            content: match value {
                BaseEvent::Closed => "Closed",
                BaseEvent::NextSlide => "NextSlide",
                BaseEvent::PrevSlide => "PrevSlide",
                BaseEvent::ScrollDown => "ScrollDown",
                BaseEvent::ScrollUp => "ScrollUp",
            }.to_string()
        }
    }
}
