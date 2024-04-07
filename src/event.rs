pub type MouseButton = glfw::MouseButton;
pub type Action = glfw::Action;
pub type WindowEvent = glfw::WindowEvent;

pub type Event = WindowEvent;

pub struct Subscriptions<M> {
    filters: Vec<fn(Event) -> Option<M>>,
}

impl<M> Subscriptions<M> {
    pub fn new(filter: fn(Event) -> Option<M>) -> Self {
        Self {
            filters: vec![filter],
        }
    }

    pub fn empty() -> Self {
        Self { filters: vec![] }
    }

    pub fn combine(subscriptions: &[Self]) -> Self {
        Self {
            filters: subscriptions
                .iter()
                .map(|s| s.filters.iter().map(|&f| f))
                .flatten()
                .collect(),
        }
    }
}

impl<M> Default for Subscriptions<M> {
    fn default() -> Self {
        Self::empty()
    }
}
