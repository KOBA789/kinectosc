use nalgebra::{Translation3, Point3, UnitQuaternion, Vector3};

pub struct Triangle {
    origin: Point3<f64>,
    v1: Vector3<f64>,
    v2: Vector3<f64>,
    center: Point3<f64>,
}

impl Triangle {
    fn new(points: [Point3<f64>; 3]) -> Self {
        let origin = points[0];
        let v1 = points[1] - origin;
        let v2 = points[2] - origin;
        let sum = points[0].coords + points[1].coords + points[2].coords;
        let center = (sum / 3.0).into();
        Self {
            origin,
            v1,
            v2,
            center,
        }
    }

    fn orientation(&self) -> UnitQuaternion<f64> {
        let norm = self.v1.cross(&self.v2);
        UnitQuaternion::face_towards(&norm, &self.v1)
    }
}

impl From<[Point3<f64>; 3]> for Triangle {
    fn from(points: [Point3<f64>; 3]) -> Self {
        Triangle::new(points)
    }
}

pub struct AvgTriangle {
    sample_buf: Vec<Vector3<f64>>,
    points: Vec<Point3<f64>>,
}

impl AvgTriangle {
    pub fn new() -> Self {
        Self {
            sample_buf: Vec::with_capacity(1000),
            points: Vec::with_capacity(3),
        }
    }

    pub fn push_sample(&mut self, sample: Point3<f64>) {
        self.sample_buf.push(sample.coords);
    }

    pub fn current_avg(&self) -> Option<Point3<f64>> {
        if self.sample_buf.is_empty() {
            return None;
        }
        let sum: Vector3<_> = self.sample_buf.iter().fold(Vector3::zeros(), std::ops::Add::add);
        let avg: Vector3<_> = sum / (self.sample_buf.len() as f64);
        Some(avg.into())
    }

    pub fn next_point(&mut self) {
        let point = match self.current_avg() {
            Some(avg) => avg,
            None => return,
        };
        self.sample_buf.clear();
        self.push_point(point);
    }

    pub fn push_point(&mut self, point: Point3<f64>) {
        if self.points.len() == 3 {
            self.points.remove(0);
        }
        self.points.push(point);
    }

    pub fn triangle(&self) -> Option<Triangle> {
        if self.points.len() != 3 {
            return None;
        }
        Some([
            self.points[0],
            self.points[1],
            self.points[2],
        ].into())
    }
}

pub fn calibrate(reference_tri: &Triangle, target_tri: &Triangle) -> (Translation3<f64>, UnitQuaternion<f64>) {
    let rotation: UnitQuaternion<_> = target_tri.orientation().rotation_to(&reference_tri.orientation());
    let rotated_target_origin = rotation.transform_point(&target_tri.origin);
    let translation: Translation3<_> = (reference_tri.origin - rotated_target_origin).into();
    (translation, rotation)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calibration() {
        check_calibration(&UnitQuaternion::identity(), &Translation3::identity());
        check_calibration(
            &UnitQuaternion::from_euler_angles(std::f64::consts::PI / 2., std::f64::consts::PI / 3., 0.), 
            &Translation3::new(1., 2., 3.)
        );
    }

    fn check_calibration(test_r: &UnitQuaternion<f64>, test_t: &Translation3<f64>) {
        let ref_pts = [
            Point3::new(1., 0., 0.),
            Point3::new(0., 2., 0.),
            Point3::new(0., 0., 3.),
        ];
        let ref_tri = Triangle::new(ref_pts);
        let tgt_pts = [
            test_t.transform_point(&test_r.transform_point(&ref_pts[0])),
            test_t.transform_point(&test_r.transform_point(&ref_pts[1])),
            test_t.transform_point(&test_r.transform_point(&ref_pts[2])),
        ];
        let tgt_tri = Triangle::new(tgt_pts);
        let (t, r) = calibrate(&ref_tri, &tgt_tri);
        let trf_pts = [
            t.transform_point(&r.transform_point(&tgt_pts[0])),
            t.transform_point(&r.transform_point(&tgt_pts[1])),
            t.transform_point(&r.transform_point(&tgt_pts[2])),
        ];
        approx::assert_relative_eq!(ref_pts[0], trf_pts[0], epsilon = 1.0e-6);
        approx::assert_relative_eq!(ref_pts[1], trf_pts[1], epsilon = 1.0e-6);
        approx::assert_relative_eq!(ref_pts[2], trf_pts[2], epsilon = 1.0e-6);
    }
}
