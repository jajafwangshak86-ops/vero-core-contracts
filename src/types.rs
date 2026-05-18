pub struct Task { pub id: u64, pub votes: u32, pub is_done: bool }
pub enum Error { NotGuardian = 1, AlreadyResolved = 2 }
