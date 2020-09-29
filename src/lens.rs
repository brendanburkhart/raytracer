use crate::linear_algebra as la;

struct ViewPort {
    width: f64,
    height: f64,
}

#[derive(Debug)]
pub struct Scope {
    position: la::Vector,
    right: la::Vector,
    up: la::Vector,
    forward: la::Vector,
}

impl Scope {
    pub fn new(target: la::Vector, position: la::Vector, roll: f64) -> Scope {
        let forward = target.subtract(&position).normalize();
        let vertical = la::Vector::new(0.0, 1.0, 0.0);

        let right: la::Vector;
        if forward.equals(&vertical) {
            right = la::Vector::new(1.0, 0.0, 0.0);
        } else {
            right = forward.cross(&vertical);
        }

        let up = right.cross(&forward);
        let up = up.rotate(-roll, &forward);

        let right = forward.cross(&up);

        Scope {
            position,
            right,
            up,
            forward,
        }
    }
}

pub trait Lens {
    fn generate_light_ray(&self, x: f64, y: f64) -> la::Ray;
}

pub struct OrthographicLens {
    view_port: ViewPort,
    scope: Scope,
}

impl OrthographicLens {
    pub fn new(width: f64, height: f64, scope: Scope) -> OrthographicLens {
        let view_port = ViewPort { width, height };

        OrthographicLens { view_port, scope }
    }
}

impl Lens for OrthographicLens {
    fn generate_light_ray(&self, x: f64, y: f64) -> la::Ray {
        let horizontal = self.scope.right.scale(x * self.view_port.width * 0.5);
        let vertical = self.scope.up.scale(y * self.view_port.height * 0.5);

        la::Ray {
            position: self.scope.position.add(&horizontal).add(&vertical),
            direction: self.scope.forward,
        }
    }
}

pub struct PerspectiveLens {
    view_port: ViewPort,
    scope: Scope,
    focal_length: f64,
}

impl PerspectiveLens {
    pub fn new(width: f64, height: f64, scope: Scope, focal_length: f64) -> PerspectiveLens {
        let view_port = ViewPort { width, height };

        PerspectiveLens {
            view_port,
            scope,
            focal_length,
        }
    }
}

impl Lens for PerspectiveLens {
    fn generate_light_ray(&self, x: f64, y: f64) -> la::Ray {
        let forward = self.scope.forward.scale(self.focal_length);
        let horizontal = self.scope.right.scale(x * self.view_port.width * 0.5);
        let vertical = self.scope.up.scale(y * self.view_port.height * 0.5);

        let direction = forward.add(&horizontal).add(&vertical).normalize();

        la::Ray {
            position: self.scope.position,
            direction,
        }
    }
}