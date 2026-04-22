use glam::DVec3;

// a Color is a DVec3 where each component is a double of range [0.0 -> 1.0]
// TODO: maybe someday convert this to a newtype (but I think I'd have to reimplement all the
// traits)
pub type Color = DVec3;
