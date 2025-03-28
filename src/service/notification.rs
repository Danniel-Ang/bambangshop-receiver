use std::thread;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use rocket::http::Status;
use crate::config::APP_CONFIG;
use crate::model::subscriber::SubscriberRequest;
use crate::utils::compose_error_response;

pub struct NotificationService;

impl NotificationService {
    #[tokio::main]
    async fn subscribe_request(product_type: String) -> Result<SubscriberRequest, String> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();

        let notification_receiver_url: String = format!(
            "{}/receive",
            APP_CONFIG.get_instance_root_url()
        );

        let payload: SubscriberRequest = SubscriberRequest {
            name: APP_CONFIG.get_instance_name().to_string(),
            url: notification_receiver_url,
        };

        let request_url: String = format!(
            "{}/notification/subscribe/{}",
            APP_CONFIG.get_publisher_root_url(),
            product_type_str
        );

        let request = Client::new()
            .post(request_url.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await;

        log::warn!("Sent subscribe request to: {}", request_url);

        match request {
            Ok(f) => match f.json::<SubscriberRequest>().await {
                Ok(x) => Ok(x),
                Err(y) => Err(compose_error_response(
                    Status::NotAcceptable,
                    y.to_string(),
                )),
            },
            Err(e) => Err(compose_error_response(
                Status::NotFound,
                e.to_string(),
            )),
        }
    }

    pub fn subscribe(product_type: &str) -> Result<SubscriberRequest, String> {
        let product_type_clone = String::from(product_type);
        return thread::spawn(move || Self::subscribe_request(product_type_clone))
            .join()
            .unwrap();
    }
}
