use self::client::{body::Body, Client};
use url::Url;

use super::{
    element::{define_elements, layout::form::Form},
    error::{ClientError, WebDynproError},
    event::Event,
};

pub struct BasicApplication {
    base_url: Url,
    name: String,
    client: Client,
}

impl<'a> BasicApplication {
    define_elements! {
        SSR_FORM: Form<'a> = "sap.client.SsrClient.form"
    }

    pub async fn new(base_url_str: &str, name: &str) -> Result<Self, WebDynproError> {
        let base_url = Url::parse(base_url_str)
            .or(Err(ClientError::InvalidBaseUrl(base_url_str.to_string())))?;
        let client = Client::new(&base_url, name).await?;
        Ok(Self::with_client(base_url, name, client)?)
    }

    pub fn with_client(base_url: Url, name: &str, client: Client) -> Result<Self, WebDynproError> {
        Ok(BasicApplication {
            base_url,
            name: name.to_owned(),
            client,
        })
    }

    pub(crate) fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(&self.base_url.as_str());
        if !url.ends_with('/') {
            url.push_str("/");
        }
        url.push_str(&self.name);
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    pub async fn send_events(&mut self, events: Vec<Event>) -> Result<(), WebDynproError> {
        let form_req = Self::SSR_FORM
            .from_body(&self.client.body)?
            .request(false, "", "", false, false)
            .or(Err(ClientError::NoSuchForm(
                Self::SSR_FORM.id().to_string(),
            )))?;
        for event in events.into_iter() {
            if !event.is_enqueable() && event.is_submitable() {
                {
                    self.client.add_event(event);
                    self.client.add_event(form_req.to_owned());
                }
                {
                    self.client.send_event(&self.base_url).await?;
                }
            } else {
                self.client.add_event(event);
            }
        }
        Ok(())
    }

    pub fn body(&self) -> &Body {
        &self.client.body
    }
}

pub mod client;
