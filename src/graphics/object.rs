use super::{vertices::DrawMode, Context};

pub trait Object {
    type Vertex;

    fn new(context: &Context) -> Self;

    fn store(
        &mut self,
        context: &Context,
        data: impl Iterator<Item = Self::Vertex>,
        mode: DrawMode,
    );

    fn draw(&self, context: &Context);

    fn delete(&self, context: &Context);

    fn draw_stream(&mut self, context: &Context, data: impl Iterator<Item = Self::Vertex>) {
        self.store(context, data, DrawMode::Stream);
        self.draw(context);
    }

    fn stream(context: &Context, data: impl Iterator<Item = Self::Vertex>)
    where
        Self: Sized,
    {
        let mut object = Self::new(context);
        object.draw_stream(context, data);
        object.delete(context);
    }
}
