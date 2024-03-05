
use bevy::{math::cubic_splines::CubicCurve, prelude::*};

pub enum EasingFunction {
    Instant,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Into<CubicSegment<Vec2>> for EasingFunction {
    fn into(self) -> CubicSegment<Vec2> {
        match self {
            EasingFunction::Instant => CubicSegment::new_bezier((1.0, 1.0), (1.0, 1.0)),
            EasingFunction::Linear => CubicSegment::new_bezier((0.0, 0.0), (1.0, 1.0)),
            EasingFunction::EaseIn => CubicSegment::new_bezier((0.42, 0.0), (1.0, 1.0)),
            EasingFunction::EaseOut => CubicSegment::new_bezier((0.0, 0.0), (0.58, 1.0)),
            EasingFunction::EaseInOut => CubicSegment::new_bezier((0.42, 0.0), (0.58, 1.0)),
        }    
    }
}
