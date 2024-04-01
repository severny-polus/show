use rand::prelude::*;
use show::{
    views::Row, Action, Bounds, Canvas, Color, Event, Length, MouseButton, Point, PointF32,
    Program, Size, Style, View, WindowEvent,
};

const DT: f32 = 0.005; // шаг времени
const DS: f32 = 2. * DT; // шаг пути
const TRAIL_LENGTH: usize = 50; // длина пути в шагах
const N: usize = 1000; // количество пылинок
const N_PER_STEP: usize = 20; // количество появляющихся пылинок за шаг
const T: usize = 60 * 10; // продолжительность жизни пылинки в кадрах

// vx = ax + by
// vy = cx + dy
struct Simulator {
    bounds: Bounds,
    size: PointF32,
    velocity: fn(PointF32) -> PointF32,
    scale: f32,
    p0: PointF32,
    points: Vec<(PointF32, usize)>,
    rng: ThreadRng,
    pressed: bool,
}

impl Simulator {
    fn new(velocity: fn(PointF32) -> PointF32, scale: f32) -> Self {
        Self {
            bounds: Bounds::zero(),
            size: PointF32::zero(),
            velocity,
            scale,
            p0: PointF32::zero(),
            points: Vec::with_capacity(N),
            rng: rand::thread_rng(),
            pressed: false,
        }
    }

    fn draw_trail(&self, canvas: &mut Canvas, steps: usize, origin: PointF32, brightness: f32) {
        let mut p = origin;
        let mut lines: Vec<(PointF32, Color)> = Vec::with_capacity(steps);
        for i in 0..steps {
            let point = p.mul(self.size.y / self.scale);
            lines.push((
                PointF32::new(point.x + self.size.x, point.y + self.size.y).mul(0.5),
                Color::white().with_alpha(i as f32 / steps as f32 * brightness),
            ));

            p = p + (self.velocity)(p).mul(DS);
        }
        canvas.draw_lines_gradient(lines.as_slice());
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
        self.size = bounds.size().to_f32()
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
                            Some((
                                p + (self.velocity)(p).mul(DT),
                                t + if p.y.abs() > self.scale
                                    || p.x.abs() > self.scale * self.size.x / self.size.y
                                {
                                    T / 60
                                } else {
                                    1
                                },
                            ))
                        }
                    })
                    .collect();
                for _ in 0..N_PER_STEP {
                    self.points.push((
                        PointF32::new(
                            (self.rng.gen::<f32>() * 2. - 1.) * self.scale * w / h,
                            (self.rng.gen::<f32>() * 2. - 1.) * self.scale * w / h,
                        ),
                        0,
                    ))
                }
            }
            Event::Window(event) => match event {
                WindowEvent::CursorPos(x, y) => {
                    self.p0 =
                        PointF32::new(2. * x as f32 - w, h - 2. * y as f32).mul(self.scale / h);
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
        for &(point, t) in &self.points {
            let brightness = 1. - (2. * t as f32 / T as f32 - 1.).powi(2);
            self.draw_trail(canvas, TRAIL_LENGTH, point, brightness)
        }

        if self.pressed {
            let steps = 5000;
            let mut p = self.p0;
            let mut lines: Vec<(PointF32, Color)> = Vec::with_capacity(steps);
            for i in 0..steps {
                let point = p.mul(self.size.y / self.scale);
                lines.push((
                    PointF32::new(point.x + self.size.x, point.y + self.size.y).mul(0.5),
                    Color::white().with_alpha((steps - i) as f32 / steps as f32),
                ));

                p = p + (self.velocity)(p).mul(DS);
            }
            canvas.draw_lines_gradient(lines.as_slice());
            // self.draw_trail(canvas, 5000, self.p0, 1.);
        }
    }
}

fn main() {
    let mut program = Program::new().unwrap();
    program
        .show(Size::Max, "Differential equations", || {
            Simulator::new(
                |p| PointF32::new(2. * p.x + 1. * p.y, 6. * p.x + 1. * p.y),
                1.,
            )
        })
        .unwrap();
}
