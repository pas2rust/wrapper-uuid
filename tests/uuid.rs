use wrapper_uuid::wrapper::{UuidTrait, WrapperUuid};

#[test]
fn gen_and_verify_uuid() {
    let uuid = WrapperUuid::create();
    let verify = WrapperUuid::verify(uuid);
    match verify {
        Ok(ok) => assert!(ok),
        Err(err) => assert!(false, "{}", err),
    };
}
