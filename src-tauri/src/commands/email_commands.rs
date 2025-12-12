use lettre::message::header::ContentType; // ✅ Import ContentType
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use tauri::command;

#[command]
pub async fn send_suggestion(
    subject: String,
    message: String,
    contact_info: Option<String>,
) -> Result<(), String> {
    // 1. Load env vars (dotenvy should be loaded at app startup, but we can double check or rely on env::var)
    // Note: In tauri main.rs or lib.rs we should make sure dotenvy::dotenv().ok(); is called.

    let smtp_host = env::var("SMTP_HOST").map_err(|_| "SMTP_HOST not set".to_string())?;
    let smtp_port_str = env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string());
    let smtp_port: u16 = smtp_port_str
        .parse()
        .map_err(|_| "Invalid SMTP_PORT".to_string())?;
    let smtp_user = env::var("SMTP_USER").map_err(|_| "SMTP_USER not set".to_string())?;
    let smtp_pass = env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD not set".to_string())?;
    let feedback_email =
        env::var("FEEDBACK_EMAIL").map_err(|_| "FEEDBACK_EMAIL not set".to_string())?;

    // 2. Format the email body
    let contact_str = contact_info.unwrap_or_else(|| "Anonymous".to_string());
    let email_body = format!(
        "New Suggestion/Complaint from Brisas App\n\nFrom: {}\n\nMessage:\n{}",
        contact_str, message
    );

    // 3. Build email
    let email = Message::builder()
        .from(
            format!("Brisas App <{}>", smtp_user)
                .parse::<lettre::message::Mailbox>()
                .map_err(|e| e.to_string())?,
        )
        .to(feedback_email
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| e.to_string())?)
        .subject(format!("Brisas App Feedback: {}", subject))
        .header(ContentType::TEXT_PLAIN) // ✅ Explicit Content-Type
        .body(email_body)
        .map_err(|e| e.to_string())?;

    // 4. Setup Transport
    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::relay(&smtp_host)
        .map_err(|e| e.to_string())?
        .port(smtp_port)
        .credentials(creds)
        .build();

    // 5. Send
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to send email: {}", e)),
    }
}
