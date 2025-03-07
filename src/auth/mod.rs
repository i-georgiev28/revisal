use async_trait::async_trait;
use leptos::prelude::*;
use crate::model::*;



#[server]
pub async fn is_auth() -> Result<bool, ServerFnError> {
    use self::*;
    use crate::model::*;
    
    dbg!("Getting the auth");
    let auth = auth()?;
    let is_authenticated = auth.is_authenticated();
    let current_user = auth.current_user.as_ref().map(|u| &u.id);
    println!("Auth check - authenticated: {}, user: {:?}", is_authenticated, current_user);
    Ok(is_authenticated)
}

#[server]
pub async fn get_user() -> Result<Option<StudentRecord>, ServerFnError> {
    use self::*;
    use crate::model::*;
    
    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    email: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use self::*;
    use crate::model::*;

    dbg!("Starting login process");
    let db = db()?;
    let auth = auth()?;

    let user: StudentRecord = StudentRecord::get_student_by_email(&db, email.clone())
        .await
        .ok_or_else(|| ServerFnError::new("User does not exist."))?;

    dbg!("Found user: {:?}", &user.id);
    let is_verified = StudentRecord::verify(&db, email.clone(), password).await;
    dbg!("Password verification result: {}", is_verified);

    if is_verified {
        dbg!("Logging in user: {}", &user.id);
        auth.login_user(user.id.clone());
        auth.remember_user(remember.is_some());
        
        // Verify the session was updated
        dbg!("Auth session state: {:?}", auth.is_authenticated());
        dbg!("Current user: {:?}", auth.current_user);
        
        leptos_axum::redirect("/app/dashboard");
        return Ok(());
    }
    Err(ServerFnError::ServerError(
        "Password does not match.".to_string(),
    ))
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
    use surrealdb::RecordId;
    use self::*;
    use crate::model::*;

    let db = db()?;
    let auth = auth()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();


    dbg!("Reached create_student");
    StudentRecord::create_student(&db, username, email.clone(), password_hashed).await?;
    dbg!("Passed create_student");

    dbg!("Now retreiving the said user");
    let user: StudentRecord = StudentRecord::get_student_by_email(&db, email)
    .await
    .ok_or_else(|| ServerFnError::new("Tf? User doesnt exist"))?;
    println!("Retrieved the user: {:?}", user);

    dbg!("Logging in");
    auth.login_user(user.id);
    auth.remember_user(remember.is_some());
    leptos_axum::redirect("/app/dashboard");
    Ok(())
}


#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use self::*;

    let auth = auth()?;
    auth.logout_user();
    leptos_axum::redirect("/app/dashboard");
    Ok(())
}

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum_session_auth::*;
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_surreal::SessionSurrealPool;
    use surrealdb::{
        engine::local::Mem,
        Surreal,
    };
    use surrealdb::engine::local::Db;
    use surrealdb::engine::any::Any;



    pub type SurrealAuthSession = AuthSession<StudentRecord, String, SessionSurrealPool<Db>, Surreal<Db>>;


#[async_trait]
impl Authentication<StudentRecord, String, Surreal<Db>> for StudentRecord {
    async fn load_user(
        user_id: String,
        db: Option<&Surreal<Db>>,
    ) -> Result<StudentRecord, anyhow::Error> {
        let db = db.unwrap();

        StudentRecord::get_student(db, user_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.id.is_empty()
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[async_trait]
impl HasPermission<Surreal<Db>> for StudentRecord {
    async fn has(&self, perm: &str, _db: &Option<&Surreal<Db>>) -> bool {
        true
    }
}

pub fn auth() -> Result<SurrealAuthSession, ServerFnError> {
    use_context::<SurrealAuthSession>()
        .ok_or_else(|| ServerFnError::ServerError("AuthSession not found.".into()))
}
pub fn db() -> Result<Surreal<Db>, ServerFnError> {
    use_context::<Surreal<Db>>()
        .ok_or_else(|| ServerFnError::ServerError("AuthSession not found.".into()))
}

}}
