use lettre::{
    error::Error as EmailError,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use std::error::Error as StdError;

pub struct EmailManager {
    from_email: Mailbox,
    to_emails: Vec<Mailbox>,
    smtp_mailer: SmtpTransport,
}

#[derive(Default)]
pub struct EmailBuilder {
    from_email: String,
    to_emails: Vec<String>,
    from_passwd: String,
    smtp_addr: String,
}

impl EmailBuilder {
    pub fn from_config() -> Self {
        // TODO: Create from config file object
        EmailBuilder::default()
    }

    pub fn sender(mut self, email: &str, passwd: &str, smtp_addr: &str) -> Self {
        self.from_email = email.to_owned();
        self.from_passwd = passwd.to_owned();
        self.smtp_addr = smtp_addr.to_owned();

        self
    }

    pub fn add_recevier(mut self, receviers: &[&str]) -> Self {
        let emails: Vec<String> = receviers.iter().map(|&s| s.to_string()).collect();
        self.to_emails.extend_from_slice(&emails);

        self
    }

    pub fn build(self) -> EmailManager {
        let from_email: Mailbox = match self.from_email.parse() {
            Ok(m) => m,
            Err(e) => {
                log::error!("Failed to parse {} as `Mailbox`.", &self.from_email);
                panic!("Panic with {}!", e);
            }
        };
        let to_emails = self
            .to_emails
            .into_iter()
            .map(|e| e.parse().unwrap())
            .collect();
        let creds = Credentials::new(from_email.email.to_string(), self.from_passwd.to_owned());

        let smtp_mailer = SmtpTransport::relay(&self.smtp_addr)
            .unwrap()
            .credentials(creds)
            .build();

        EmailManager {
            from_email,
            to_emails,
            smtp_mailer,
        }
    }
}

impl EmailManager {
    pub fn builder() -> EmailBuilder {
        EmailBuilder::default()
    }

    fn create_msg(
        &self,
        subject: &str,
        body: &str,
        to_email: &Mailbox,
    ) -> Result<Message, EmailError> {
        Message::builder()
            .from(self.from_email.to_owned())
            .to(to_email.to_owned())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(body))
    }

    pub fn send(&self, subject: &str, body: &str) -> Result<(), Box<dyn StdError>> {
        for email in &self.to_emails {
            let msg = self.create_msg(subject, body, email)?;

            self.smtp_mailer.send(&msg)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_email() {
        let email_mgr = EmailManager::builder()
            .sender(
                "Quant Trader <15670556067@163.com>",
                "IVIECOXSPGMWSDRX",
                "smtp.163.com",
            )
            .add_recevier(&["Changfeng Lou <louchangfeng@outlook.com>"])
            .build();
        let res = email_mgr.send("Test Message", "<h1> ETHUSDT: 1802.15 </h1>");

        match res {
            Ok(_) => log::info!("Successfully send email!"),
            Err(e) => log::error!("Failed to send email with: {}", e),
        }
    }
}
