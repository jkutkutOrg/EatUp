use rocket::form::{FromFormField, ValueField};
use rocket::serde::Deserialize;
use uuid::Uuid;

pub struct UuidWrapper(Uuid);
impl<'v> FromFormField<'v> for UuidWrapper {
    fn from_value(field: ValueField<'v>) -> rocket::form::Result<'v, Self> {
        let uuid = Uuid::parse_str(field.value).unwrap();
        Ok(UuidWrapper(uuid))
    }
}
impl<'de> Deserialize<'de> for UuidWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uuid = Uuid::deserialize(deserializer)?;
        Ok(UuidWrapper(uuid))
    }
}
impl std::fmt::Debug for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}