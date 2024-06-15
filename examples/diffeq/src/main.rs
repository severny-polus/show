use std::iter::zip;

use rand::prelude::*;
use show::{
    graphics::{Color, Context, DrawMode, PointArray, PointColorArray, Shape, VertexArray},
    Action, Bounds, Drawer, Event, Length, MouseButton, Point, Program, Size, View,
};

const DT: f32 = 0.001; // шаг времени
const TRAIL_DISTORTION: f32 = 1.; // удлиннение пути засчёт срезания шага
const TRAIL_LENGTH: usize = 200; // длина пути в шагах
const PARTICLES_PER_FRAME: usize = 5; // количество меняющихся пылинок за шаг
const LIFETIME: usize = PARTICLES_TOTAL / PARTICLES_PER_FRAME; // продолжительность жизни пылинки в кадрах
const STEPS_OUTSIDE: usize = LIFETIME / 60; // количество шагов, на которые перепрыгивает возраст пылинки, пока её начало вне границ экрана

const PARTICLES_TOTAL: usize = 2000; // общее количество пылинок

struct Simulator {
    velocity: fn(Point<f32>) -> Point<f32>,
    max_y: f32,
}

// vx = ax + by
// vy = cx + dy
struct SimulatorDrawer {
    velocity: fn(Point<f32>) -> Point<f32>,
    max_y: f32,

    bounds: Bounds,

    size: Point<f32>,
    rng: ThreadRng,
    pressed: bool,

    p0: Point<f32>,
    particles: [Point<f32>; PARTICLES_TOTAL],
    trails: Vec<PointColorArray>,
    initialized_particles: usize,
    particles_counter: usize,

    trail_data: [(Point<f32>, Color); TRAIL_LENGTH],

    axes: PointArray,
}

impl View for Simulator {
    fn new_drawer(&self, context: &Context) -> Box<dyn Drawer<()>> {
        Box::new(SimulatorDrawer {
            velocity: self.velocity,
            max_y: self.max_y,

            bounds: Bounds::zero(),

            size: Point::<f32>::zero(),
            rng: rand::thread_rng(),
            pressed: false,

            p0: Point::<f32>::zero(),

            particles: [Point::<f32>::zero(); PARTICLES_TOTAL],
            trails: (0..PARTICLES_TOTAL)
                .map(|_| PointColorArray::new(context))
                .collect(),
            initialized_particles: 0,
            particles_counter: 0,

            trail_data: [(Point::<f32>::zero(), Color::transparent()); TRAIL_LENGTH],

            axes: PointArray::new(context),
        })
    }
}

fn fade_fn(s: f32, t: f32) -> f32 {
    let s1 = t.powi(2);
    let s2 = 1. - (1. - t).powi(2);
    if s < s1 || s2 < s {
        0.
    } else {
        1. - (1. - 2. * s).powi(2)
    }
}

impl SimulatorDrawer {
    fn draw_trails(&mut self, context: &Context) {
        for i in 0..self.initialized_particles {
            let mut p = self.particles[i].clone();
            let age = ((self.particles_counter + PARTICLES_TOTAL - i) % PARTICLES_TOTAL)
                / PARTICLES_PER_FRAME;
            let t = age as f32 / LIFETIME as f32; // параметр времени жизни [0;1)

            // переменная длина пылинки
            for j in 0..TRAIL_LENGTH {
                let point = p.mul(self.size.y / self.max_y);

                let x = 1. - 1. / (1. + (self.velocity)(p).len());
                let color = Color::from_hsv(240. + 180. * x, 1., 1.);
                let s = j as f32 / TRAIL_LENGTH as f32; // параметр хвостового звена [0;1)
                self.trail_data[j] = (
                    (point + self.size).mul(0.5),
                    color.with_alpha(fade_fn(s, t)),
                );

                p = p + (self.velocity)(p).mul(DT);
            }

            self.trails[i].draw_stream(context, self.trail_data.into_iter(), Shape::LineStrip);
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

    fn set_bounds(&mut self, context: &Context, bounds: Bounds) {
        self.bounds = bounds;
        self.size = bounds.size().to_f32();
        self.axes.buffer_data(
            context,
            [
                Point::new(self.bounds.min.x, self.bounds.center().y).to_f32(),
                Point::new(self.bounds.max.x, self.bounds.center().y).to_f32(),
                Point::new(self.bounds.center().x, self.bounds.min.y).to_f32(),
                Point::new(self.bounds.center().x, self.bounds.max.y).to_f32(),
            ]
            .into_iter(),
            DrawMode::Dynamic,
        )
    }

    fn process(&mut self, event: Event) -> Option<()> {
        match event {
            event => match event {
                Event::CursorPos(x, y) => {
                    self.p0 = Point::new(2. * x as f32 - self.size.x, self.size.y - 2. * y as f32)
                        .mul(self.max_y / self.size.y);
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

    fn draw(&mut self, context: &Context) {
        context.set_color(Color::from_hsva(0., 0., 1., 0.5));
        self.axes.draw(context, Shape::Lines);

        self.draw_trails(context);

        if self.pressed {
            let steps = 5000;
            let mut p = self.p0;
            let mut points: Vec<Point<f32>> = Vec::with_capacity(steps);

            for _ in 0..steps {
                let point = p.mul(self.size.y / self.max_y);
                points.push((point + self.size).mul(0.5));

                p = p + (self.velocity)(p).mul(DT);
            }
            let colors: Vec<Color> = (0..points.len())
                .map(|i| Color::white().with_alpha((points.len() - i) as f32 / points.len() as f32))
                .collect();

            PointColorArray::draw_once(context, zip(points, colors), Shape::LineStrip)
        }

        // движение хвоста
        // for i in 0..self.initialized_particles {
        //     let p = self.particles[i];
        //     let velocity = (self.velocity)(p);
        //     self.particles[i] = p + velocity.mul(DT);
        // }
        for i in self.particles_counter..self.particles_counter + PARTICLES_PER_FRAME {
            // let multiplier = (self.size.x / self.size.y).max(self.size.y / self.size.x);
            let out_of_bounds = 1.5;
            self.particles[i] = Point::new(
                (self.rng.gen::<f32>() * 2. - 1.) * self.max_y * out_of_bounds * self.size.x
                    / self.size.y,
                (self.rng.gen::<f32>() * 2. - 1.) * self.max_y * out_of_bounds,
            );
        }
        self.particles_counter =
            (self.particles_counter + PARTICLES_PER_FRAME) % self.particles.len();
        if self.initialized_particles < PARTICLES_TOTAL {
            self.initialized_particles += PARTICLES_PER_FRAME;
        }
    }
}

fn main() {
    let mut program = Program::new().unwrap();
    program
        .show(Size::Max, "Differential equations", || Simulator {
            velocity: |p| Point::new(2. * p.x - 1. * p.y, 6. * p.x - 1. * p.y),
            max_y: 1.,
        })
        .unwrap();
}
