mod reverse;
mod rotate;
mod tea;

pub use self::reverse::ReverseKeySchedule;
pub use self::rotate::SimpleRotateKeySchedule;
pub use self::tea::TeaKeySchedule;

pub trait KeySchedule<K> {
    fn next_key(&mut self) -> K;
}
