use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{self, UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::{self, remove_token_cookie, Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.route("/api/logoff", post(api_logoff_handler))
		.with_state(mm)
}

// region:    --- Login
async fn api_login_handler(
	mm: State<ModelManager>,
	cookies: Cookies,
	payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_login_handler", "HANDLER");

	// TODO: Implement real db/auth logic.
	// if payload.username != "demo1" || payload.pwd != "welcome" {
	// 	return Err(Error::LoginFail);
	// }

	let axum::Json(LoginPayload {
		username,
		pwd: pwd_clear,
	}) = payload;

	let root_ctx = Ctx::root_ctx();

	//--Get the user
	let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;

	let user_id = user.id;

	//-- Validate the password
	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd { user_id });
	};

	pwd::validate_pwd(
		&EncryptContent {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear.clone(),
		},
		&pwd,
	)
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	// FIXME: Implement real auth-token generation/signature.
	// cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

	//Set the web token
	web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}
// endregion: --- Login

// region:    --- LogOff

async fn api_logoff_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>
)-> Result<Json<Value>>{
	debug!("{:<12} - api_logoff_handler","HANDLER");
	let should_lofoff = payload.logoff;

	if should_lofoff{
		remove_token_cookie(&cookies)?;
	}

	//Create the success body.
	let body = Json(json!({
		"result":{
			"logged_off":should_lofoff
		}
	}));
	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload{
	logoff:bool,
}
// endregion: --- LogOff


