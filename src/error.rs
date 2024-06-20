use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound { id: u64 },
    NoCookieWasFoundAndAuthIsRequired,
    AuthFailTokenWrongFormat,
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
        }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("Error: {:<12} - {self:?}", "INTO_RES");

        match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login Failed").into_response(),
            Error::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::NOT_FOUND, "Ticket Not Found").into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        }
    }
}
