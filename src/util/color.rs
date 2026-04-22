use glam::DVec3;

// a Color is a DVec3 where each component is a double of range [0.0 -> 1.0]
// TODO: maybe someday convert this to a newtype (but I think I'd have to reimplement all the
// traits)
pub type Color = DVec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
