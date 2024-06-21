use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound { id: u64 },
    NoCookieWasFoundAndAuthIsRequired,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl Error {
    pub fn client_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL),
            Error::NoCookieWasFoundAndAuthIsRequired => {
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH)
            }
            Error::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::LoginFail => write!(f, "Login Fail"),
            Error::TicketDeleteFailIdNotFound { id } => {
                write!(f, "Ticket Delete Fail: Id Not Found: {}", id)
            }
            Error::NoCookieWasFoundAndAuthIsRequired => {
                write!(f, "No Cookie Was Found And Auth Is Required")
            }
            Error::AuthFailTokenWrongFormat => write!(f, "Auth Fail Token Wrong Format"),
            Error::AuthFailCtxNotInRequestExt => write!(f, "Auth Fail Ctx Not In Request Ext"),
        }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        println!("Error: {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
