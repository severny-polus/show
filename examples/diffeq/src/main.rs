use show::{views::Row, Bounds, Color, Event, Length, Point, Program, Style, View};

// vx = ax + by
// vy = cx + dy
struct Simulator {
    bounds: Bounds,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    dr: f64,
    x0: f64,
    y0: f64,
}

impl Simulator {
    fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
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
        let w = self.bounds.width() as f64;
        let h = self.bounds.height() as f64;
        match event {
            Event::CursorPos(x, y) => {
                self.x0 = (2. * x - w) / h;
                self.y0 = -(2. * y - h) / h;
            }
            _ => {}
        }
        None
    }

    fn draw(&self, canvas: &mut show::Canvas) {
        let w = self.bounds.width() as f64;
        let h = self.bounds.height() as f64;
        let mut x = self.x0;
        let mut y = self.y0;
        let n: usize = 100;
        let mut points: Vec<Point> = Vec::with_capacity(n);
        let mut colors: Vec<Color> = Vec::with_capacity(n);
        for i in 0..n+1 {
            points.push(Point::new(
                ((x * h + w) / 2.) as i32,
                ((y * h + h) / 2.) as i32,
            ));
            colors.push(Color::white().with_alpha((255 * (n - i) / n) as u8));

            x += (self.a * x + self.b * y) * self.dr;
            y += (self.c * x + self.d * y) * self.dr;
        }
        canvas.draw_lines_gradient(points.as_slice(), colors.as_slice());
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
