use rand::prelude::*;
use show::{
    views::Row, Action, Bounds, Color, Event, Length, MouseButton, Point, PointF32, Program, Style,
    View, WindowEvent,
};

const DT: f32 = 0.005; // шаг времени
const SCALE: f32 = 3.; // ордината, соответствующая верхней границе экрана
const STEPS: usize = 5000;
const N: usize = 200; // количество пылинок за кадр
const T: usize = 120; // продолжительность жизни пылинки в кадрах

// vx = ax + by
// vy = cx + dy
struct Simulator {
    bounds: Bounds,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    p0: PointF32,
    points: Vec<(PointF32, usize)>,
    rng: ThreadRng,
    pressed: bool,
    eigenvectors: Option<[PointF32; 2]>,
}

impl Simulator {
    fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            bounds: Bounds::zero(),
            a,
            b,
            c,
            d,
            p0: PointF32::zero(),
            points: Vec::new(),
            rng: rand::thread_rng(),
            pressed: false,
            eigenvectors: None,
        }
    }

    fn velocity(&self, p: PointF32) -> PointF32 {
        PointF32::new(
            f32::ln(5. - 2. * p.x() - 2. * p.y()),
            f32::exp(p.x() * p.y()) - 1.,
        )
    }

    fn eigenvectors(a: f32, b: f32, c: f32, d: f32) -> Option<[PointF32; 2]> {
        if b == 0. {
            None
        } else {
            let b1 = -(a + d);
            let c1 = a * d - b * c;
            let d1 = b1 * b1 - 4. * c1;
            if d1 < 0. {
                None
            } else {
                let p = 0.5 * (-b1 - d1.sqrt());
                let q = 0.5 * (-b1 + d1.sqrt());
                Some([
                    PointF32::new(1., (p - a) / b),
                    PointF32::new(1., (q - a) / b),
                ])
            }
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
            Event::Frame => {
                self.points = self
                    .points
                    .iter()
                    .filter_map(|&(p, t)| {
                        if t >= T {
                            None
                        } else {
                            Some((p + self.velocity(p).mul(DT), t + 1))
                        }
                    })
                    .collect();
                for _ in 0..N {
                    self.points.push((
                        PointF32::new(
                            (self.rng.gen::<f32>() * 2. - 1.) * SCALE * 2. * w / h,
                            (self.rng.gen::<f32>() * 2. - 1.) * SCALE * 2. * w / h,
                        ),
                        0,
                    ))
                }
            }
            Event::Window(event) => match event {
                WindowEvent::CursorPos(x, y) => {
                    self.p0 = PointF32::new(2. * x as f32 - w, h - 2. * y as f32).mul(SCALE / h);
                }
                WindowEvent::MouseButton(button, action, _modifiers) => {
                    if button == MouseButton::Button1 {
                        match action {
                            Action::Press => self.pressed = true,
                            Action::Release => self.pressed = false,
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
        }
        None
    }

    fn draw(&self, canvas: &mut show::Canvas) {
        let w = self.bounds.width() as f32;
        let h = self.bounds.height() as f32;

        match self.eigenvectors {
            Some([v, u]) => {
                let c = PointF32::new(w / 2., h / 2.);
                let a = 0.25;
                canvas.draw_lines(
                    &[c - v.mul(w / 2.), c + v.mul(w / 2.)],
                    Color::new(1., 0., 0., a),
                );
                canvas.draw_lines(
                    &[c - u.mul(w / 2.), c + u.mul(w / 2.)],
                    Color::new(0., 0., 1., a),
                );
            }
            _ => {}
        }

        let points: Vec<(PointF32, Color)> = self
            .points
            .iter()
            .map(|&(p, t)| {
                let p = p.mul(h / SCALE);
                (
                    PointF32::new(p.x() + w, p.y() + h).mul(0.5),
                    Color::white().with_alpha(1. - (2. * t as f32 / T as f32 - 1.).powi(2)),
                )
            })
            .collect();
        canvas.draw_points(points.as_slice());

        if self.pressed {
            let mut p = self.p0;
            let mut lines: Vec<(PointF32, Color)> = Vec::with_capacity(STEPS);
            for i in 0..STEPS {
                let point = p.mul(h / SCALE);
                lines.push((
                    PointF32::new(point.x() + w, point.y() + h).mul(0.5),
                    Color::white().with_alpha((STEPS - i) as f32 / STEPS as f32),
                ));

                p = p + self.velocity(p).mul(DT);
            }
            canvas.draw_lines_gradient(lines.as_slice());
        }
    }
}

fn main() {
    let mut program = Program::new().unwrap();
    program
        .show("Differential equations", || {
            Simulator::new(-4., -9., 3., 8.)
        })
        .unwrap();
}
