use uuid::Uuid;

pub struct WrapperUuid;
pub trait UuidTrait {
    fn create() -> String;
    fn verify(uuid: String) -> Result<bool, String>;
}

impl UuidTrait for WrapperUuid {
    fn create() -> String {
        Uuid::new_v4().to_string()
    }
    fn verify(uuid: String) -> Result<bool, String> {
        match Uuid::parse_str(uuid.as_str()) {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }
}
