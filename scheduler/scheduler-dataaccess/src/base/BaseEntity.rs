use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait BaseEntity {
    fn id(&self) -> Uuid;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn created_by(&self) -> Option<Uuid>;
    fn updated_by(&self) -> Option<Uuid>;
}
