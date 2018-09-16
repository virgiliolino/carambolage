use super::glium;
use super::glium::{glutin, Frame, Surface};
use super::model::Model;
use super::nalgebra::{Matrix4, Point2, Vector2, Vector3};

pub(super) struct Car {
    pos: Point2<f32>,
    _rot: f32,
    vel: Vector2<f32>,
    force: Vector2<f32>,
    mass: f32,

    model: Model,
}

impl Car {
    pub fn new(
        pos: Point2<f32>,
        mass: f32,
        color: Vector3<f32>,
        display: &glium::Display,
    ) -> Car {
        assert!(mass > 0.);
        Car {
            pos,
            _rot: 0.,
            vel: Vector2::new(0., 0.),
            force: Vector2::new(0., 0.),
            mass,

            model: Model::new(color, display),
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(&mut self, time_step: f32) {
        //assert!(time_step > 0.);
        self.pos += self.vel * time_step
            + self.force / (2. * self.mass) * time_step.powi(2);
        self.vel += self.force / self.mass * time_step;
    }

    pub(super) fn draw(
        &self,
        target: &mut Frame,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
    ) {
        //target.draw()
    }
}

#[cfg(test)]
mod test {
    use super::Car;
    use super::Model;
    use nalgebra::{Point2, Vector2, Vector3};

    #[test]
    fn go_car_1() {
        let mut car = Car {
            pos: Point2::new(0., 0.),
            _rot: 0.,
            vel: Vector2::new(1., 0.),
            force: Vector2::new(0., 0.),
            mass: 1.,

            model: Model::new(Vector3::new(1., 0., 0.)),
        };
        car.run(1.);
        assert_eq!(car.pos, Point2::new(1., 0.));
    }

    #[test]
    fn go_car_2() {
        let mut car = Car {
            pos: Point2::new(0., 0.),
            _rot: 0.,
            vel: Vector2::new(0., 0.),
            force: Vector2::new(1., 0.),
            mass: 1.,

            model: Model::new(Vector3::new(1., 0., 0.)),
        };
        car.run(2.);
        assert_eq!(car.pos, Point2::new(2., 0.));
        assert_eq!(car.vel, Vector2::new(2., 0.));
    }
}
