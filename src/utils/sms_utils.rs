use reqwest::Client;

pub async fn send_sms(
    phone_number: &str,
    verification_code: &str,
    config: &crate::config::AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post(&format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            config.twilio_account_sid
        ))
        .basic_auth(&config.twilio_account_sid, Some(&config.twilio_auth_token))
        .form(&[
            ("To", phone_number),
            ("From", &config.twilio_from_phone_number),
            ("Body", &format!("Your verification code is: {}", verification_code)),
        ])
        .send()
        .await?;

    if response.status().is_success() {
        println!("SMS sent successfully!");
    } else {
        println!("Failed to send SMS: {:?}", response.text().await?);
    }

    Ok(())
}