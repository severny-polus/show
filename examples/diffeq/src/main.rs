use show::{views::Row, Bounds, Color, Event, Length, Point, PointF32, Program, Style, View};

// vx = ax + by
// vy = cx + dy
struct Simulator {
    bounds: Bounds,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    dr: f32,
    x0: f32,
    y0: f32,
}

impl Simulator {
    fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            bounds: Bounds::zero(),
            a,
            b,
            c,
            d,
            dr: 0.01,
            x0: 0.5,
            y0: 0.5,
        }
    }
}

impl View for Simulator {
    fn width(&self) -> show::Length {
        Length::Fill
    }

    fn height(&self) -> show::Length {
        Length::Fill
    }

    fn set_bounds(&mut self, bounds: show::Bounds) {
        self.bounds = bounds;
    }

    fn process(&mut self, event: show::Event) -> Option<()> {
        let w = self.bounds.width() as f32;
        let h = self.bounds.height() as f32;
        match event {
            Event::CursorPos(x, y) => {
                self.x0 = (2. * x as f32 - w) / h;
                self.y0 = -(2. * y as f32 - h) / h;
            }
            _ => {}
        }
        None
    }

    fn draw(&self, canvas: &mut show::Canvas) {
        let w = self.bounds.width() as f32;
        let h = self.bounds.height() as f32;
        let mut x = self.x0;
        let mut y = self.y0;
        let n: usize = 100;
        let mut points: Vec<(PointF32, Color)> = Vec::with_capacity(n);
        for i in 0..n {
            points.push((
                PointF32::new((x * h + w) / 2., (y * h + h) / 2.),
                Color::white().with_alpha((n - i) as f32 / n as f32),
            ));

            x += (self.a * x + self.b * y) * self.dr;
            y += (self.c * x + self.d * y) * self.dr;
        }
        canvas.draw_points(points.as_slice());
    }
}

fn main() {
    let mut program = Program::new().unwrap();
    program
        .show("Differential equations", || {
            Box::new(Simulator::new(3., -2., 4., -1.))
        })
        .unwrap();
}
