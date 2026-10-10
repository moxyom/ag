use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::CookieJar;

use crate::{
    AppState,
    jwt::{self, JwtData},
};

impl<S> FromRequestParts<S> for JwtData
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar.get("auth").ok_or(StatusCode::UNAUTHORIZED)?.value();

        Ok(jwt::verify(token, &state.jwt_secret))
    }
}
