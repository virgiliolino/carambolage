// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
use super::controller::Controller;
use super::model::Model;

use nalgebra::{zero, Matrix4, Vector3};
use time::Duration;

pub struct Car {
    /// The center of mass of the car
    ///
    /// Note that the car does not rotate around this point. It rotates around
    /// the rear axle.
    pub center_of_mass: Vector3<f32>,
    /// The forward orientation of the car
    pub orientation: Vector3<f32>,
    /// Mass of the car in kg
    mass: f32,
    /// Distance of the front axle from the center of mass in meter
    dist_front_axle: f32,
    /// Distance of the rear axle from the center of mass in meter
    dist_rear_axle: f32,
    /// The graphical model of the car
    pub model: Model,
}

impl Car {
    pub fn new(center_of_mass: Vector3<f32>, mass: f32) -> Car {
        let mut car: Car = Default::default();
        car.center_of_mass = center_of_mass;
        if mass > 1. {
            car.mass = mass;
        }

        car
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(&mut self, delta_time: Duration, controller: Option<Controller>) {
        if let Some(ct) = controller {
            let dt = delta_time.num_milliseconds() as f32 / 1_000.;

            // accel:  0.0 - None
            //         1.0 - Pedal to the metal
            //        -1.0 - Emergency brake
            let accel = ct.get_y_axis();
            // steer:  0.0 - Forward
            //         1.0 - Full right
            //        -1.0 - Full left
            // * accel to prevent steering a non moving car.
            let steer = ct.get_x_axis() * accel;

            self.orientation[2] -= steer * dt * 3.5;

            let rot_mat = Matrix4::new_rotation(self.orientation);
            let mut forward = Vector3::new(0f32, 1., 0.).to_homogeneous();
            forward = rot_mat * forward;
            // Set homogeneous coordinate to 0 or unwrap() will panic.
            forward[3] = 0.;

            self.center_of_mass += Vector3::from_homogeneous(forward).unwrap() * accel * dt * 10.;
        }
    }

    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        // x,y-axis rotation are fixed to 0. No rollovers!
        let rotation = Matrix4::from_euler_angles(0., 0., self.orientation[2]);
        let translation = Matrix4::new_translation(&self.center_of_mass);
        let model = translation * rotation;
        let mvp = projection * view * model;
        self.model.draw(&mvp);
    }
}

impl Default for Car {
    fn default() -> Car {
        Car {
            center_of_mass: zero(),
            orientation: zero(),
            mass: 1.,
            dist_front_axle: 1.,
            dist_rear_axle: 1.,
            model: Model::new(),
        }
    }
}
