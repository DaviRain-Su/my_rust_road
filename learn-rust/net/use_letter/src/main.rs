use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};
use std::thread;
use std::time::Duration;
fn main() {

    let email = EmailBuilder::new()
        .from(Mailbox::new("17366503261@163.com".to_string()))
        .to(Mailbox::new("1129584904@qq.com".to_string()))
        .subject("Test Email")
        .body("This is a test email! from Program to send!!!")
        .build()
        .unwrap();

    let creds = Credentials::new("17366503261".to_string(), "su1025185920".to_string());

    let mut mailer = SmtpClient::new_simple("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .transport();
    
    for _i in 0..100 {
        let result = mailer.send(email.clone().into());

    
        if result.is_ok() {
            println!("Email sent");
        }else {
            println!("Could not send email: {:?}", result);
        }
    
        assert!(result.is_ok());
        thread::sleep(Duration::from_secs(60));
    }
   
}
