use nalgebra::{Vector3, Point3, UnitQuaternion, Quaternion, Rotation3};

const JOINT_COUNT: usize = k4a::joint_id::K4ABT_JOINT_COUNT as usize;

#[derive(Debug, Clone)]
pub struct SmoothParams {
    smoothing: f64,
    correction: f64,
    prediction: f64,
    jitter_radius: f64,
    max_deviation_radius: f64,
}

impl Default for SmoothParams {
    fn default() -> Self {
        Self {
            smoothing: 0.25,
            correction: 0.25,
            prediction: 0.25,
            jitter_radius: 0.03,
            max_deviation_radius: 0.05,
            //jitter_angle: 
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilteredJoint {
    pub raw_position: Point3<f64>,
    pub filtered_position: Point3<f64>,
    pub trend: Vector3<f64>,
    pub predicted_position: Point3<f64>,
    pub raw_orientation: UnitQuaternion<f64>,
    pub filtered_orientation: UnitQuaternion<f64>,
    pub orientation_trend: Rotation3<f64>,
    pub predicted_orientation: UnitQuaternion<f64>,
    pub frame_count: u64,
}

impl Default for FilteredJoint {
    fn default() -> Self {
        Self {
            raw_position: Point3::origin(),
            filtered_position: Point3::origin(),
            trend: Vector3::zeros(),
            predicted_position: Point3::origin(),
            raw_orientation: UnitQuaternion::identity(),
            filtered_orientation: UnitQuaternion::identity(),
            orientation_trend: Rotation3::identity(),
            predicted_orientation: UnitQuaternion::identity(),
            frame_count: 0,
        }
    }
}

impl FilteredJoint {
    pub fn update(&mut self, mut params: SmoothParams, joint: &k4a::Joint) {
        if joint.confidence_level.0 == 1 {
            params.jitter_radius *= 2.;
            params.max_deviation_radius *= 2.;
        }
        if joint.confidence_level.0 == 0 {
            self.frame_count = 0;
        }
        let prev_filtered_position: Point3<_> = self.filtered_position;
        let prev_trend: Vector3<_> = self.trend;
        let prev_raw_position: Point3<_> = self.raw_position;
        let prev_filtered_orientation = self.filtered_orientation;
        let prev_orientation_trend = self.orientation_trend;
        let prev_raw_orientation = self.raw_orientation;
        let raw_position: Point3<_> = k4a_float3_to_vector3f64(&joint.position).into();
        let raw_orientation: UnitQuaternion<_> = k4a_quaternion_to_unit_quaternion_f64(&joint.orientation);
        
        if self.frame_count == 0 {
            self.filtered_position = raw_position;
            self.trend = Vector3::zeros();
            self.filtered_orientation = raw_orientation;
            self.orientation_trend = Rotation3::identity();
            self.frame_count += 1;
        } else if self.frame_count == 1 {
            self.filtered_position = nalgebra::center(&raw_position, &prev_raw_position);
            let diff = self.filtered_position.coords - prev_filtered_position.coords;
            self.trend = diff.lerp(&prev_trend, params.correction);
            self.filtered_orientation = raw_orientation.nlerp(&prev_raw_orientation, 0.5);
            let rot_to: UnitQuaternion<f64> = prev_filtered_orientation.rotation_to(&self.filtered_orientation);
            self.orientation_trend = rot_to.nlerp(&prev_orientation_trend.into(), params.correction).into();
            self.frame_count += 1;
        } else {
            let jitter: f64 = nalgebra::distance(&raw_position, &prev_filtered_position);
            if jitter <= params.jitter_radius {
                self.filtered_position = raw_position.coords.lerp(
                    &prev_filtered_position.coords, 
                    jitter / params.jitter_radius
                ).into();
            } else {
                self.filtered_position = raw_position;
            }

            self.filtered_position = self.filtered_position.coords.lerp(
                &prev_filtered_position.coords, 
                params.smoothing
            ).into();
            let diff = self.filtered_position.coords - prev_filtered_position.coords;
            self.trend = diff.lerp(&prev_trend, params.correction);

            // no jitter filter for orientation
            self.filtered_orientation = raw_orientation;
            self.filtered_orientation = self.filtered_orientation.slerp(
                &prev_filtered_orientation, 
                params.smoothing
            );
            let rot_to: UnitQuaternion<f64> = prev_filtered_orientation.rotation_to(&self.filtered_orientation);
            self.orientation_trend = rot_to.nlerp(&prev_orientation_trend.into(), params.correction).into();
        }
        self.predicted_position = (
            self.filtered_position.coords
            + self.trend * params.prediction
        ).into();
        let deviation: f64 = nalgebra::distance(&self.predicted_position, &raw_position);
        if deviation > params.max_deviation_radius {
            self.predicted_position = self.predicted_position.coords.lerp(
                &raw_position.coords,
                params.max_deviation_radius / deviation
            ).into();
        }
    
        self.predicted_orientation = self.orientation_trend.powf(params.prediction) * self.filtered_orientation;

        self.raw_position = raw_position;
        self.raw_orientation = raw_orientation;
    }
}

#[derive(Debug, Clone)]
pub struct KinectJointFilter {
    params: SmoothParams,
    pub joints: [FilteredJoint; JOINT_COUNT],
}

impl KinectJointFilter {
    pub fn new(params: SmoothParams) -> Self {
        Self {
            params,
            joints: Default::default(),
        }
    }

    pub fn update(&mut self, skeleton: &k4a::Skeleton) {
        for (idx, joint) in skeleton.joints.iter().enumerate() {
            self.joints[idx].update(self.params.clone(), joint);
        }
    }
}

fn k4a_float3_to_vector3f64(k4a_float3: &k4a::Float3) -> Vector3<f64> {
    Vector3::new(
        k4a_float3.x as f64,
        k4a_float3.y as f64,
        k4a_float3.z as f64,
    )
}

fn k4a_quaternion_to_unit_quaternion_f64(k4a_q: &k4a::Quaternion) -> UnitQuaternion<f64> {
    UnitQuaternion::from_quaternion(Quaternion::new(
        k4a_q.w as f64,
        k4a_q.x as f64,
        k4a_q.y as f64,
        k4a_q.z as f64,
    ))
}
