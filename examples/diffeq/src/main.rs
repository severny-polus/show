use rand::prelude::*;
use show::{
    Action, Bounds, Canvas, Color, Event, Length, MouseButton, Point, Program, Size, View,
    WindowEvent,
};

const DT: f32 = 0.001; // шаг времени
const TRAIL_LENGTH: usize = 100; // длина пути в шагах
const PARTICLES_PER_FRAME: usize = 20; // количество появляющихся пылинок за шаг
const T: usize = 60 * 10; // продолжительность жизни пылинки в кадрах
const STEPS_WHILE_OUT: usize = T / 60; // количество шагов, на которые перепрыгивает возраст пылинки, пока её начало вне границ экрана

// vx = ax + by
// vy = cx + dy
struct Simulator {
    bounds: Bounds,
    size: Point<f32>,
    velocity: fn(Point<f32>) -> Point<f32>,
    scale: f32,
    p0: Point<f32>,
    points: Vec<(Point<f32>, usize)>,
    rng: ThreadRng,
    pressed: bool,

    lines: [(Point<f32>, Color); TRAIL_LENGTH],
}

impl Simulator {
    fn new(velocity: fn(Point<f32>) -> Point<f32>, scale: f32) -> Self {
        Self {
            bounds: Bounds::zero(),
            size: Point::<f32>::zero(),
            velocity,
            scale,
            p0: Point::<f32>::zero(),
            points: Vec::new(),
            rng: rand::thread_rng(),
            pressed: false,

            lines: [(Point::<f32>::zero(), Color::transparent()); TRAIL_LENGTH],
        }
    }

    fn draw_trails(&mut self, canvas: &mut Canvas) {
        for i in 0..self.points.len() {
            let (mut p, t) = self.points[i].clone();
            let brightness = 1. - (2. * t as f32 / T as f32 - 1.).powi(2);

            for j in 0..TRAIL_LENGTH {
                let point = p.mul(self.size.y / self.scale);
                self.lines[j] = (
                    Point::new(point.x + self.size.x, point.y + self.size.y).mul(0.5),
                    Color::white().with_alpha(j as f32 / TRAIL_LENGTH as f32 * brightness),
                );

                p = p + (self.velocity)(p).mul(DT);
            }
            if p.y.abs() > self.scale || p.x.abs() > self.scale * self.size.x / self.size.y {
                if t == 0 {
                    self.points[i].1 += T;
                    continue;
                }
                self.points[i].1 = t + STEPS_WHILE_OUT;
            }

            canvas.draw_lines_gradient(self.lines.as_slice());
            // TODO: split points and color calculations
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
        self.size = bounds.size().to_f32()
    }

    fn process(&mut self, event: show::Event) -> Option<()> {
        match event {
            event => match event {
                WindowEvent::CursorPos(x, y) => {
                    self.p0 = Point::new(2. * x as f32 - self.size.x, self.size.y - 2. * y as f32)
                        .mul(self.scale / self.size.y);
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

    fn draw(&mut self, canvas: &mut show::Canvas) {
        self.draw_trails(canvas);

        if self.pressed {
            let steps = 5000;
            let mut p = self.p0;
            let mut lines: Vec<(Point<f32>, Color)> = Vec::with_capacity(steps);
            for i in 0..steps {
                let point = p.mul(self.size.y / self.scale);
                lines.push((
                    Point::new(point.x + self.size.x, point.y + self.size.y).mul(0.5),
                    Color::white().with_alpha((steps - i) as f32 / steps as f32),
                ));

                p = p + (self.velocity)(p).mul(DT);
            }
            canvas.draw_lines_gradient(lines.as_slice());
        }

        self.points = self
            .points
            .iter()
            .filter_map(|&(p, t)| {
                if t >= T {
                    None
                } else {
                    Some((p + (self.velocity)(p).mul(DT), t + 1))
                }
            })
            .collect();
        for _ in 0..PARTICLES_PER_FRAME {
            self.points.push((
                Point::new(
                    (self.rng.gen::<f32>() * 2. - 1.) * self.scale * self.size.x / self.size.y,
                    (self.rng.gen::<f32>() * 2. - 1.) * self.scale * self.size.x / self.size.y,
                ),
                0,
            ))
        }
    }
}

fn main() {
    let mut program = Program::new().unwrap();
    program
        .show(Size::Max, "Differential equations", || {
            Simulator::new(|p| Point::new(2. * p.x + 1. * p.y, 6. * p.x + 1. * p.y), 1.)
        })
        .unwrap();
}
