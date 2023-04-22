pub mod register {
    use reqwest::Client;
    use serde_json::{from_str, json};

    use crate::{
        api::{
            limits::LimitType,
            schemas::schemas::{ErrorBody, RegisterSchema},
        },
        errors::InstanceServerError,
        instance::{Instance, Token},
    };

    impl Instance {
        /**
        Registers a new user on the Spacebar server.
        # Arguments
        * `register_schema` - The [`RegisterSchema`] that contains all the information that is needed to register a new user.
        # Errors
        * [`InstanceServerError`] - If the server does not respond.
         */
        pub async fn register(
            &mut self,
            register_schema: &RegisterSchema,
        ) -> Result<Token, InstanceServerError> {
            let json_schema = json!(register_schema);
            let limited_requester = &mut self.requester;
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/auth/register";
            let request_builder = client.post(endpoint_url).body(json_schema.to_string());
            let response = limited_requester
                .send_request(request_builder, LimitType::AuthRegister)
                .await;
            if response.is_none() {
                return Err(InstanceServerError::NoResponse);
            }

            let response_unwrap = response.unwrap();
            let status = response_unwrap.status();
            let response_text_string = response_unwrap.text().await.unwrap();
            if status.is_client_error() {
                let error: ErrorBody = from_str(&response_text_string).unwrap();
                return Err(InstanceServerError::InvalidFormBodyError {
                    error: error.errors.errors.iter().next().unwrap().code.clone(),
                });
            }
            return Ok(Token {
                token: response_text_string,
            });

            /*
            Things to do:
            Check out the serde error. Maybe make a seperate project to find out how flatten works
             */
        }
    }
}

#[cfg(test)]
mod test {
    use crate::api::schemas::schemas::RegisterSchema;
    use crate::instance::Instance;
    use crate::limit::LimitedRequester;
    use crate::URLBundle;
    #[tokio::test]
    async fn test_registration() {
        let urls = URLBundle::new(
            "http://localhost:3001/api".to_string(),
            "http://localhost:3001".to_string(),
            "http://localhost:3001".to_string(),
        );
        let limited_requester = LimitedRequester::new(urls.get_api().to_string()).await;
        let mut test_instance = Instance::new(urls.clone(), limited_requester)
            .await
            .unwrap();
        let reg = RegisterSchema::new(
            "Test".to_string(),
            None,
            true,
            Some("me@mail.xy".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        println!("{}", test_instance.register(&reg).await.unwrap());
    }
}
