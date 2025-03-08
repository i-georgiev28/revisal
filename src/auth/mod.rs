use async_trait::async_trait;
use leptos::prelude::*;
use crate::model::*;

#[server]
pub async fn is_auth() -> Result<bool, ServerFnError> {
    use crate::model::*;
    use cookie::Cookie;

    let db = db()?;
    let cookies = cookies()?;
    let email_cookie = cookies.get("email");
    let password_cookie = cookies.get("password");

    if email_cookie.is_none() || password_cookie.is_none() {
        cookies.remove(Cookie::new("email", ""));
        cookies.remove(Cookie::new("password", ""));
        return Ok(false);
    }

    let email_cookie = email_cookie.unwrap();
    let password_cookie = password_cookie.unwrap();

    let email = email_cookie.value();
    let password = password_cookie.value();

    dbg!(format!("Verifying password: {}", password.to_string()));
    let is_verified = StudentRecord::verify(&db, email.to_string(), password.to_string()).await;
        
    Ok(is_verified)
}

#[server]
pub async fn get_user() -> Result<Option<StudentRecord>, ServerFnError> {
    use crate::model::*;
    use cookie::Cookie;

    let db = db()?;
    let cookies = cookies()?;

    let email_cookie = cookies.get("email");
    let password_cookie = cookies.get("password");

    if email_cookie.is_none() || password_cookie.is_none() {
        cookies.remove(Cookie::new("email", ""));
        cookies.remove(Cookie::new("password", ""));
        return Ok(None);
    }

    let email_cookie = email_cookie.unwrap();
    let password_cookie = password_cookie.unwrap();

    let email = email_cookie.value();
    let password = password_cookie.value();

    let is_verified = StudentRecord::verify(&db, email.to_string(), password.to_string()).await;
    if !is_verified {
        cookies.remove(Cookie::new("email", ""));
        cookies.remove(Cookie::new("password", ""));
        return Ok(None);
     }

    let user: Option<StudentRecord> = StudentRecord::get_student_by_email(&db, email.to_string()).await;
    Ok(user)
}

#[server(Login, "/api")]
pub async fn login(
    email: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use crate::model::*;
    use cookie::Cookie;
    use cookie::time::Duration;

    let cookies = cookies()?;

    let db = db()?;
    let user: StudentRecord = StudentRecord::get_student_by_email(&db, email.clone())
        .await
        .ok_or_else(|| ServerFnError::new("User does not exist."))?;

    let is_verified = StudentRecord::verify(&db, email.clone(), password).await;

    if is_verified {
    // TODO: Implement cookie
    cookies.add(Cookie::build(("email", user.email.clone())).path("/").http_only(true).max_age(Duration::days(1)).build());
    cookies.add(Cookie::build(("password", user.password.clone())).path("/").http_only(true).max_age(Duration::days(1)).build());
    leptos_axum::redirect("/app/dashboard");
        return Ok(());
    }
    Err(ServerFnError::ServerError("Password does not match.".to_string()))
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    email: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use bcrypt::{hash, DEFAULT_COST};
    use crate::model::*;
    use cookie::Cookie;
    use cookie::time::Duration;

    let cookies = cookies()?;
    let db = db()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError("Passwords did not match.".to_string()));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();
    StudentRecord::create_student(&db, username, email.clone(), password_hashed).await?;

    let user: StudentRecord = StudentRecord::get_student_by_email(&db, email)
        .await
        .ok_or_else(|| ServerFnError::new("Failed to create user"))?;

    cookies.add(Cookie::build(("email", user.email.clone())).path("/").http_only(true).max_age(Duration::days(1)).build());
    cookies.add(Cookie::build(("password", user.password.clone())).path("/").http_only(true).max_age(Duration::days(1)).build());

    leptos_axum::redirect("/app/dashboard");
    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use cookie::Cookie;

    let cookies = cookies()?;
    cookies.remove(Cookie::new("email", ""));
    cookies.remove(Cookie::new("password", ""));
    // TODO: Implement cookie

    leptos_axum::redirect("/");
    Ok(())
}

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    use surrealdb::{
        engine::local::Db,
        Surreal,
    };
    use tower_cookies::Cookies;


    pub fn cookies() -> Result<Cookies, ServerFnError> {
        use_context::<Cookies>()
            .ok_or_else(|| ServerFnError::ServerError("No cookie context found".into()))
    }

    pub fn db() -> Result<Surreal<Db>, ServerFnError> {
        use_context::<Surreal<Db>>()
            .ok_or_else(|| ServerFnError::ServerError("Database connection not found.".into()))
    }
}}
