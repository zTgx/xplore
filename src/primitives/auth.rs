use serde::{Deserialize, Serialize};


pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub bearer_token: String,
    pub two_factor_secret: Option<String>,
}

impl AuthConfig {
    pub fn new(bearer_token: String) -> Self {
        Self {
            username: None,
            password: None,
            email: None,
            bearer_token,
            two_factor_secret: None,
        }
    }

    pub fn with_credentials(
        mut self,
        username: String,
        password: String,
        email: Option<String>,
    ) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self.email = email;
        self
    }
}


#[derive(Debug)]
pub enum SubtaskType {
    LoginJsInstrumentation,
    LoginEnterUserIdentifier,
    LoginEnterPassword,
    LoginAcid,
    AccountDuplicationCheck,
    LoginTwoFactorAuthChallenge,
    LoginEnterAlternateIdentifier,
    LoginSuccess,
    DenyLogin,
    Unknown(String),
}

impl From<&str> for SubtaskType {
    fn from(s: &str) -> Self {
        match s {
            "LoginJsInstrumentationSubtask" => Self::LoginJsInstrumentation,
            "LoginEnterUserIdentifierSSO" => Self::LoginEnterUserIdentifier,
            "LoginEnterPassword" => Self::LoginEnterPassword,
            "LoginAcid" => Self::LoginAcid,
            "AccountDuplicationCheck" => Self::AccountDuplicationCheck,
            "LoginTwoFactorAuthChallenge" => Self::LoginTwoFactorAuthChallenge,
            "LoginEnterAlternateIdentifierSubtask" => Self::LoginEnterAlternateIdentifier,
            "LoginSuccessSubtask" => Self::LoginSuccess,
            "DenyLoginSubtask" => Self::DenyLogin,
            other => Self::Unknown(other.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FlowInitRequest {
    pub flow_name: String,
    pub input_flow_data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct FlowTaskRequest {
    pub flow_token: String,
    pub subtask_inputs: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct FlowResponse {
    pub flow_token: String,
    pub subtasks: Option<Vec<Subtask>>,
}

#[derive(Debug, Deserialize)]
pub struct Subtask {
    pub subtask_id: String,
}

