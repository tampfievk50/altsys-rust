use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Trait representing the common audit columns shared by all SeaORM entity models.
/// Every entity Model should implement this trait so that generic code can
/// access audit fields uniformly, similar to inheriting from a base entity.
pub trait BaseEntity {
    fn id(&self) -> Uuid;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn created_by(&self) -> Option<Uuid>;
    fn updated_by(&self) -> Option<Uuid>;

    /// Returns `(created_at, updated_at)` both initialized to the same current
    /// timestamp. Use this when building a new `ActiveModel` so the two fields
    /// are guaranteed to be equal on creation.
    fn init_timestamps() -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        (now, now)
    }
}
