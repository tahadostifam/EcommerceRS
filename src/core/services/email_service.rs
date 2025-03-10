use std::collections::HashMap;

pub trait EmailService: Send + Sync {
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        template_name: &str,
        context: HashMap<String, String>,
    ) -> Result<(), String>;
}

pub struct EmailServiceImpl {}

pub struct EmailServiceDevelImpl {}

pub fn new_email_service_devel() -> impl EmailService {
    EmailServiceDevelImpl {}
}

impl EmailService for EmailServiceDevelImpl {
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        template_name: &str,
        context: HashMap<String, String>,
    ) -> Result<(), String> {
        println!("Sending email (development mode):");
        println!("  To: {}", to);
        println!("  Subject: {}", subject);
        println!("  Template: {}", template_name);
        println!("  Context: {:?}", context);
        Ok(())
    }
}
