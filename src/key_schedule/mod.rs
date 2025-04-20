mod reverse;
mod tea;

pub use self::reverse::ReverseKeySchedule;
pub use self::tea::TeaKeySchedule;

pub trait KeySchedule {
    type SubKey;

    fn next_key(&mut self) -> Self::SubKey;
}
