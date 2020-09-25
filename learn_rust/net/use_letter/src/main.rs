use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};

fn main() {

    let email = EmailBuilder::new()
        .from(Mailbox::new("17366503261@163.com".to_string()))
        .to(Mailbox::new("71205902002@stu.ecnu.edu.cn".to_string()))
        .subject("Test")
        .body("This is a test email!")
        .build()
        .unwrap();

    let creds = Credentials::new("17366503261".to_string(), "*************".to_string());

    let mut mailer = SmtpClient::new_simple("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());

    
    if result.is_ok() {
        println!("Email sent");
    }else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}
