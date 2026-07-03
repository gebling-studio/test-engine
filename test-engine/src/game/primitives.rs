#[cfg(feature = "2d")]
mod _2d {
    pub type Point = crate::gm::flat::Point;
    pub type Size = crate::gm::flat::Size;
    pub(crate) type Rotation = f32;
}

#[cfg(feature = "3d")]
mod _3d {
    pub type Point = crate::gm::volume::Point3;
    pub type Size = crate::gm::volume::Size3;
    pub type Point = crate::gm::volume::Quaternion;
}

#[cfg(feature = "2d")]
pub use _2d::*;
#[cfg(feature = "3d")]
pub use _3d::*;
