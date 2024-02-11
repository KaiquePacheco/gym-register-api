#[path = "../src/main.rs"]
mod application;

#[path = "setup.rs"]
mod setup;

use rocket::{
    async_test,
    http::{ContentType, Status},
};

#[async_test]
async fn is_sign_in_returning_token() {
    let client = setup::client().await;

    let cases = [
        ("carlos@example.com", "Hello"),
        ("maria@hotmail.com", "maria1234"),
    ];

    for (email, password) in cases {
        let res = client
            .post("/auth/signin")
            .header(ContentType::Form)
            .body(&format!("email={email}&password={password}"))
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Accepted);
    }
}
