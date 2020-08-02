use k4a::{Skeleton, joint_id};

const PARENT_JOINT: [joint_id::Type; joint_id::K4ABT_JOINT_COUNT as usize] = [
    joint_id::K4ABT_JOINT_COUNT,
    joint_id::K4ABT_JOINT_PELVIS,
    joint_id::K4ABT_JOINT_SPINE_NAVEL,
    joint_id::K4ABT_JOINT_SPINE_CHEST,
    joint_id::K4ABT_JOINT_SPINE_CHEST,
    joint_id::K4ABT_JOINT_CLAVICLE_LEFT,
    joint_id::K4ABT_JOINT_SHOULDER_LEFT,
    joint_id::K4ABT_JOINT_ELBOW_LEFT,
    joint_id::K4ABT_JOINT_WRIST_LEFT,
    joint_id::K4ABT_JOINT_HAND_LEFT,
    joint_id::K4ABT_JOINT_WRIST_LEFT,
    joint_id::K4ABT_JOINT_SPINE_CHEST,
    joint_id::K4ABT_JOINT_CLAVICLE_RIGHT,
    joint_id::K4ABT_JOINT_SHOULDER_RIGHT,
    joint_id::K4ABT_JOINT_ELBOW_RIGHT,
    joint_id::K4ABT_JOINT_WRIST_RIGHT,
    joint_id::K4ABT_JOINT_HAND_RIGHT,
    joint_id::K4ABT_JOINT_WRIST_RIGHT,
    joint_id::K4ABT_JOINT_PELVIS,
    joint_id::K4ABT_JOINT_HIP_LEFT,
    joint_id::K4ABT_JOINT_KNEE_LEFT,
    joint_id::K4ABT_JOINT_ANKLE_LEFT,
    joint_id::K4ABT_JOINT_PELVIS,
    joint_id::K4ABT_JOINT_HIP_RIGHT,
    joint_id::K4ABT_JOINT_KNEE_RIGHT,
    joint_id::K4ABT_JOINT_ANKLE_RIGHT,
    joint_id::K4ABT_JOINT_NECK,
    joint_id::K4ABT_JOINT_HEAD,
    joint_id::K4ABT_JOINT_HEAD,
    joint_id::K4ABT_JOINT_HEAD,
    joint_id::K4ABT_JOINT_HEAD,
    joint_id::K4ABT_JOINT_HEAD,
];

pub struct GlobalSkelton {
    
}

impl GlobalSkelton {
    pub fn new(skelton: &Skeleton) {
        //skelton.joints
    }
}
