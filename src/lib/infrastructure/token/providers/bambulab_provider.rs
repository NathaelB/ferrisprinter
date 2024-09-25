use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde::Serialize;

use crate::domain::token::{
    models::token::{CreateTokensError, Token, Tokens},
    ports::provider_token_service::ProviderTokenService,
};

pub struct BambuLabProviderTokenService {
    http_client: Client,
    #[allow(dead_code)]
    api_url: String,
    login_url: String,
}

impl BambuLabProviderTokenService {
    pub fn new(api_url: String, login_url: String) -> Self {
        Self {
            http_client: Client::new(),
            login_url,
            api_url,
        }
    }

    fn extract_token_from_cookie(
        headers: &reqwest::header::HeaderMap,
        cookie_name: &str,
    ) -> Option<String> {
        headers
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|header| header.to_str().ok())
            .filter_map(|cookie_str| cookie::Cookie::parse(cookie_str).ok())
            .find(|cookie| cookie.name() == cookie_name)
            .map(|cookie| cookie.value().to_string())
    }
}

#[derive(Serialize)]
struct AuthPayload {
    account: String,
    password: String,
}

impl ProviderTokenService for BambuLabProviderTokenService {
    async fn authenticate(
        &self,
        username: String,
        password: String,
    ) -> Result<Tokens, CreateTokensError> {
        let payload = AuthPayload {
            account: username,
            password,
        };

        let response = self
            .http_client
            .post(&self.login_url)
            .header(USER_AGENT, "ferris-printer")
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|_| CreateTokensError::ProviderError)?;

        let headers = response.headers();
        let refresh_token =
            BambuLabProviderTokenService::extract_token_from_cookie(headers, "refreshToken")
                .ok_or(CreateTokensError::ProviderError)?;
        let access_token =
            BambuLabProviderTokenService::extract_token_from_cookie(headers, "token")
                .ok_or(CreateTokensError::ProviderError)?;

        let refresh_token =
            Token::new(&refresh_token).map_err(|_| CreateTokensError::InvalidToken)?;
        let access_token =
            Token::new(&access_token).map_err(|_| CreateTokensError::InvalidToken)?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use reqwest::header::{HeaderValue, SET_COOKIE};

    use super::BambuLabProviderTokenService;
    use crate::domain::token::ports::provider_token_service::ProviderTokenService;

    fn mock_headers_with_cookies() -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            SET_COOKIE,
            HeaderValue::from_str("refreshToken=mock_refresh_token; HttpOnly").unwrap(),
        );
        headers.append(
            SET_COOKIE,
            HeaderValue::from_str("token=mock_access_token; HttpOnly").unwrap(),
        );

        headers
    }

    fn mock_headers_without_cookies() -> reqwest::header::HeaderMap {
        reqwest::header::HeaderMap::new()
    }

    #[tokio::test]
    async fn test_extract_token_from_cookie_success() {
        let headers = mock_headers_with_cookies();
        println!("Headers: {:?}", headers);

        let refresh_token =
            BambuLabProviderTokenService::extract_token_from_cookie(&headers, "refreshToken");
        let access_token =
            BambuLabProviderTokenService::extract_token_from_cookie(&headers, "token");

        assert_eq!(refresh_token, Some("mock_refresh_token".to_string()));
        assert_eq!(access_token, Some("mock_access_token".to_string()));
    }

    #[tokio::test]
    async fn test_extract_token_from_cookie_missing() {
        let headers = mock_headers_without_cookies();

        // Act: Tenter d'extraire des tokens inexistants
        let refresh_token =
            BambuLabProviderTokenService::extract_token_from_cookie(&headers, "refreshToken");
        let access_token =
            BambuLabProviderTokenService::extract_token_from_cookie(&headers, "token");

        // Assert: Vérifier que les tokens ne sont pas trouvés
        assert!(refresh_token.is_none());
        assert!(access_token.is_none());
    }

    #[tokio::test]
    async fn test_authenticate_sucess() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/api/sign-in/form")
                .header("Content-Type", "application/json");

            then.status(200)
                .header("set-cookie", "refreshToken=mock_refresh_token; HttpOnly")
                .header("set-cookie", "token=mock_access_token; HttpOnly")
                .json_body("{}"); // Simulate empty JSON body
        });
        let service =
            BambuLabProviderTokenService::new(server.url("/"), server.url("/api/sign-in/form"));

        let result = service
            .authenticate("test".to_string(), "test".to_string())
            .await;

        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.refresh_token.as_str(), "mock_refresh_token");
        assert_eq!(tokens.access_token.as_str(), "mock_access_token");

        mock.assert();
    }
}
