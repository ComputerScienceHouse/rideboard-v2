use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::json;

pub struct PingClient {
    client: Client,
    join_route: String,
    leave_route: String,
    add_route: String,
    remove_route: String,
}

impl PingClient {
    pub fn new(
        token: String,
        join_route: String,
        leave_route: String,
        add_route: String,
        remove_route: String,
    ) -> Result<PingClient> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token}"))?,
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(PingClient {
            client,
            join_route,
            leave_route,
            add_route,
            remove_route,
        })
    }

    pub async fn send_join(&self, to: &str, join: &str, event: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.join_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("{join} joined your ride to \"{event}\".")
            }))
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_leave(&self, to: &str, leave: &str, event: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.leave_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("{leave} left your ride \"{event}\".")
            }))
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_add(&self, to: &str, driver: &str, event: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.add_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("You have been added to {driver}'s ride to \"{event}\" by the driver.")
            }))
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_remove(&self, to: &str, driver: &str, event: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.remove_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("You have been removed from {driver}'s ride to \"{event}\" by the driver.")
            }))
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_opening(&self, to: &str, driver: &str, event: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.remove_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("{driver} has an opening in their ride to \"{event}\"!")
            }))
            .send()
            .await?;
        Ok(())
    }
}
