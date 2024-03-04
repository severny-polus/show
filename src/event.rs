

pub type Event = glfw::WindowEvent;

pub struct Subscriptions<M> {
    filters: Vec<fn(Event) -> Option<M>>,
}

impl<M> Subscriptions<M> {
    fn new(filter: fn(Event) -> Option<M>) -> Self {
        Self {
            filters: vec![filter],
        }
    }

    fn combine(subscriptions: &mut [Self]) -> Self {
        let mut filters = Vec::new();
        for s in subscriptions {
            filters.append(&mut s.filters);
        }
        Self { filters }
    }
}

impl<M> Default for Subscriptions<M> {
    fn default() -> Self {
        Self {
            filters: Vec::new(),
        }
    }
}