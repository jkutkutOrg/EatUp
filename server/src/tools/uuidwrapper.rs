use rocket::form::{FromFormField, ValueField};
use rocket::serde::Deserialize;
use rocket::request::FromParam;
use uuid::Uuid;

pub struct UuidWrapper(Result<Uuid, String>);

impl UuidWrapper {
    // pub fn unwrap(&self) -> Uuid {
    //     match self.0 {
    //         Ok(uuid) => uuid,
    //         Err(_) => panic!("tried to unwrap invalid uuid")
    //     }
    // }

    pub fn get(self) -> Result<Uuid, String> {
        self.0
    }
}

impl<'v> FromFormField<'v> for UuidWrapper {
    fn from_value(field: ValueField<'v>) -> rocket::form::Result<'v, Self> {
        Ok(match Uuid::parse_str(field.value) {
            Ok(uuid) => UuidWrapper(Ok(uuid)),
            Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
        })
    }
}

impl<'r> FromParam<'r> for UuidWrapper {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        // let uuid = Uuid::parse_str(param).unwrap();
        // Ok(UuidWrapper(uuid))
        // match Uuid::parse_str(param) {
        //     Ok(uuid) => Ok(UuidWrapper(uuid)),
        //     Err(_) => Err("invalid uuid")
        // }
        Ok(match Uuid::parse_str(param) {
            Ok(uuid) => UuidWrapper(Ok(uuid)),
            Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
        })
    }
}

impl<'de> Deserialize<'de> for UuidWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // let uuid = Uuid::deserialize(deserializer)?;
        // Ok(UuidWrapper(uuid))
        Ok(match Uuid::deserialize(deserializer) {
            Ok(uuid) => UuidWrapper(Ok(uuid)),
            Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
        })
    }
}
impl std::fmt::Debug for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}