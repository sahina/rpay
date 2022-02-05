use std::collections::HashMap;

use rpay_core::vars;

#[actix_rt::test]
#[ignore]
async fn sms_test() -> anyhow::Result<()> {
    let token = vars::plivo_token();
    let id = vars::plivo_auth_id();

    let api_base = "https://api.plivo.com/v1";
    let send_sms_url = format!("/Account/{}/Message/", id);
    let api_url = format!("{}{}", api_base, send_sms_url);

    let src = "18432770719";
    let destination = "12016285333";

    let request_body = HashMap::from([
        ("src", src),
        ("dst", destination),
        ("text", "hello there"),
        ("type", "sms")
    ]);

    let bearer = format!("{}:{}", id, token).into_bytes();
    let encoded = base64::encode(&bearer);

    let res = reqwest::Client::new()
        .post(api_url)
        .json(&request_body)
        .header("Authorization", format!("Basic {}", encoded))
        .send()
        .await?;

    assert_eq!(202, res.status());

    Ok(())
}