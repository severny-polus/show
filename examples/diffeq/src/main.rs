use std::iter::zip;

use rand::prelude::*;
use show::{
    graphics::{objects::PolylineGradient, Object},
    Action, Bounds, Color, Context, Drawer, Event, Length, MouseButton, Point, Program, Size, View,
};

const DT: f32 = 0.0015; // шаг времени
const TRAIL_LENGTH: usize = 100; // длина пути в шагах
const PARTICLES_PER_FRAME: usize = 20; // количество появляющихся пылинок за шаг
const T: usize = 60 * 5; // продолжительность жизни пылинки в кадрах
const STEPS_OUTSIDE: usize = T / 60; // количество шагов, на которые перепрыгивает возраст пылинки, пока её начало вне границ экрана

const PARTICLES_TOTAL: usize = 1000;

struct Simulator {
    velocity: fn(Point<f32>) -> Point<f32>,
    scale: f32,
}

// vx = ax + by
// vy = cx + dy
struct SimulatorDrawer {
    velocity: fn(Point<f32>) -> Point<f32>,
    scale: f32,

    bounds: Bounds,
    size: Point<f32>,
    p0: Point<f32>,
    points: Vec<(Point<f32>, usize)>,
    rng: ThreadRng,
    pressed: bool,

    trail_points: [Point<f32>; TRAIL_LENGTH],
    trail_colors: [Color; TRAIL_LENGTH],
}

impl View for Simulator {
    fn new_drawer(&self, context: &mut Context) -> Box<dyn Drawer<()>> {
        Box::new(SimulatorDrawer {
            velocity: self.velocity,
            scale: self.scale,

            bounds: Bounds::zero(),
            size: Point::<f32>::zero(),
            p0: Point::<f32>::zero(),
            points: Vec::new(),
            rng: rand::thread_rng(),
            pressed: false,

            trail_points: [Point::<f32>::zero(); TRAIL_LENGTH],
            trail_colors: [Color::transparent(); TRAIL_LENGTH],
        })
    }
}

impl SimulatorDrawer {
    fn draw_trails(&mut self, context: &mut Context) {
        for i in 0..self.points.len() {
            let (mut p, t) = self.points[i].clone();
            let brightness = 1. - (2. * t as f32 / T as f32 - 1.).powi(2);

            for j in 0..TRAIL_LENGTH {
                let point = p.mul(self.size.y / self.scale);
                self.trail_points[j] =
                    Point::new(point.x + self.size.x, point.y + self.size.y).mul(0.5);

                p = p + (self.velocity)(p).mul(DT);

                let x = 1. - 1. / (1. + (self.velocity)(p).len());
                let color = Color::from_hsv(240. + 180. * x, 1., 1.);
                self.trail_colors[j] =
                    color.with_alpha(j as f32 / TRAIL_LENGTH as f32 * brightness);
            }
            if p.y.abs() > self.scale || p.x.abs() > self.scale * self.size.x / self.size.y {
                if t == 0 {
                    self.points[i].1 += T;
                    continue;
                }
                self.points[i].1 = t + STEPS_OUTSIDE;
            }

            PolylineGradient::stream(context, zip(self.trail_points, self.trail_colors));
        }
    }
}

impl Drawer for SimulatorDrawer {
    fn width(&self) -> show::Length {
        Length::Fill
    }

    fn height(&self) -> show::Length {
        Length::Fill
    }

    fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
        self.size = bounds.size().to_f32()
    }

    fn process(&mut self, event: Event) -> Option<()> {
        match event {
            event => match event {
                Event::CursorPos(x, y) => {
                    self.p0 = Point::new(2. * x as f32 - self.size.x, self.size.y - 2. * y as f32)
                        .mul(self.scale / self.size.y);
                }
                Event::MouseButton(button, action, _modifiers) => {
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

    fn draw(&mut self, context: &mut Context) {
        self.draw_trails(context);

        if self.pressed {
            let steps = 5000;
            let mut p = self.p0;
            let mut points: Vec<Point<f32>> = Vec::with_capacity(steps);

            for i in 0..steps {
                let point = p.mul(self.size.y / self.scale);
                points.push(Point::new(point.x + self.size.x, point.y + self.size.y).mul(0.5));

                p = p + (self.velocity)(p).mul(DT);
            }
            let colors: Vec<Color> = (0..points.len())
                .map(|i| Color::white().with_alpha((points.len() - i) as f32 / points.len() as f32))
                .collect();

            PolylineGradient::stream(context, zip(points, colors))
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
        .show(Size::Max, "Differential equations", || Simulator {
            velocity: |p| Point::new(2. * p.x + 1. * p.y, 6. * p.x + 1. * p.y),
            scale: 1.,
        })
        .unwrap();
}
