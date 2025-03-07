use leptos::{prelude::*, server};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StudentRecord {
    pub id: String,
 
    pub username: String,
    pub email: String,
    pub password: String, 
    pub schedule: u32,
}

#[server(CurrentStudent, "/api")]
pub async fn current_student() -> Result<Option<StudentRecord>, ServerFnError> {
    use crate::auth::*;
    let auth = auth();
    if auth.is_err() {
        return Ok(None);
    }
    Ok(auth.unwrap().current_user)
}

cfg_if::cfg_if!{
if #[cfg(feature = "ssr")] {
    use surrealdb::{engine::local::Db, RecordId, Surreal};
    use surrealdb::Response;

    impl StudentRecord {
        pub async fn get_student(db: &Surreal<Db>, user_id: String) -> Option<Self> {

            let student: Option<StudentRecord> = db.select(("students", user_id.clone())).await.unwrap().map(SurrealStudentRecord::into_user);
            dbg!("Student {} found by id: {:?}", user_id.clone(), student.clone());
            return student
        }

        pub async fn create_student(
            db: &Surreal<Db>,
            username: String,
            email: String,
            password: String,
            // schedule: u32
        ) -> surrealdb::Result<Response> {        
            // Define the query with parameter placeholders.
            let query = r#"
                CREATE students CONTENT {
                    username: $username,
                    email: $email,
                    password: $password,
                    schedule: $schedule
                }
            "#;
    
            // Build the parameters object.
            let params = serde_json::json!({
                "username": username,
                "email": email,
                "password": password,
                "schedule": 1,
            });
    
            // Execute the query with the provided parameters.
            let response = db.query(query).bind(params).await?;
            Ok(response)
        }
    
        pub async fn get_student_by_email(db: &Surreal<Db>, email: String) -> Option<Self> {
            let student: Option<SurrealStudentRecord> = db
                .query("SELECT id, username, email, password, schedule FROM students where email = $email")
                .bind(("email", email.clone()))
                .await
                .unwrap()
                .take(0)
                .unwrap();
    
            student.map(SurrealStudentRecord::into_user)
        }
    
        pub async fn verify(db: &Surreal<Db>, email: String, password: String) -> bool {
            let student: Option<SurrealStudentRecord> = db
                .query("SELECT id, email, username, password, schedule FROM students where email = $email")
                .bind(("email", email.clone()))
                .await
                .unwrap()
                .take(0)
                .unwrap();
    
            if let Some(student) = student {

                dbg!("Verifying password for user: {}", student.id);
                match bcrypt::verify(password, &student.password) {
                    Ok(matches) => {
                        dbg!("Password verification result: {}", matches);
                        return matches;
                    },
                    Err(e) => {
                        dbg!("Password verification error: {:?}", e);
                        return false;
                    }
                }
            }


            false
        }
    
        pub async fn create_students_table(db: &Surreal<Db>) {
            db.query(
            "   DEFINE TABLE students SCHEMAFULL; 
                DEFINE FIELD username ON TABLE students TYPE string;
                DEFINE FIELD email ON TABLE students TYPE string;
                DEFINE FIELD password ON TABLE students TYPE string;
                DEFINE FIELD schedule ON TABLE students TYPE int;
            ",
            );
    
    
            // ALSO CREATE TEST STUDENT
            let user_id = RecordId::from_table_key::<&str, String>("students", "1".into());
            let user: Option<StudentRecord> = StudentRecord::get_student(db, "1".into()).await;
            if user.is_none() {
                let _: Result<Option<SurrealStudentRecord>, surrealdb::Error> = db
                    .create("students")
                    .content(SurrealStudentRecord {
                        id: user_id.clone(),
                        username: "Test User 1" .to_owned(),
                        email: "test1@example.com".to_owned(),
                        password: hash("test".to_string(), DEFAULT_COST).unwrap(),
                        schedule: 12,
                    })
                    .await;
            }
            
        }
    
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SurrealStudentRecord {
        pub id: RecordId,
        pub username: String,
        pub email: String,
        pub password: String,
        pub schedule: u32,
    }
    impl SurrealStudentRecord {
        pub fn into_user(self) -> StudentRecord {
            let id = self.id.key().to_string();
            let mut id = id.chars();
            // id.next();
            // id.next_back();
            StudentRecord {
                id: id.as_str().to_string(),
                username: self.username,
                email: self.email,
                password: self.password,
                schedule: self.schedule
            }
        }
    }

}}


