use leptos::{prelude::*, server, server_fn::ServerFn};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};

use super::StudentRecord;

#[derive(Serialize, Deserialize, Clone)]
pub struct EventRecord {
    pub id: String,
    pub owner_id: String,

    pub name: String,
    pub description: String,

    pub label: String,

    pub timestamp: i64
}

impl EventRecord {}

#[server(GetEvents, "/api")]
pub async fn get_events() -> Result<Vec<EventRecord>, ServerFnError> {
    use self::*;
    use crate::model::*;
    use crate::auth::*;
    let db = db()?;
    let user = auth()?;

    Ok(EventRecord::get_events_for_user(&db, user.id).await?)
}

#[server(CreateEvent, "/api")]
pub async fn create_event(input: (String, String, String, NaiveDateTime)) -> Result<(), ServerFnError> {
    use self::*;
    use crate::model::*;
    use crate::auth::*;

    let (name, description,label, timestamp) = input;

    let db = db()?;
    let user = auth()?;
    EventRecord::create_event(&db, user.id, name, description, label, timestamp).await?;
    Ok(())
}

#[server(DeleteEvent, "/api")]
pub async fn delete_event(event_id: String) -> Result<(), ServerFnError> {
    use self::*;
    use crate::model::*;
    use crate::auth::*;

    let db = db()?;
    let user = auth()?;

    EventRecord::delete_event(&db, event_id).await?;
    Ok(())
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use surrealdb::{Response, Value};
        use surrealdb::{engine::local::Db, RecordId, Surreal};
        use serde_json::json;

        impl EventRecord {
            pub async fn create_event_table(db: &Surreal<Db>) {
                db.query("
                    DEFINE TABLE events SCHEMAFULL;
                    DEFINE FIELD owner_id ON TABLE events TYPE record<students>;
                    DEFINE FIELD name ON TABLE events TYPE string;
                    DEFINE FIELD description ON TABLE events TYPE string;
                    DEFINE FIELD label ON TABLE events TYPE string;
                    DEFINE FIELD timestamp ON TABLE events TYPE int;
                ");
            }

            pub async fn create_event(
                db: &Surreal<Db>,
                owner_id: String,
                name: String,
                description: String,
                label: String,
                timestamp: NaiveDateTime
            ) -> surrealdb::Result<Response> {        
                // Define the query with parameter placeholders.
                let query = r#"
                    CREATE events CONTENT {
                        owner_id: $owner_id,
                        name: $name,
                        description: $description,
                        label: $label,
                        timestamp: $timestamp
                    }
                "#;
        
                // Build the parameters object.
                let params = json!({
                    "owner_id": owner_id,
                    "name": name,
                    "description": description,
                    "label": label,
                    "timestamp": timestamp.and_utc().timestamp(),
                });
        
                // Execute the query with the provided parameters.
                let response = db.query(query).bind(params).await?;
                Ok(response)
            }

            pub async fn get_events_for_user(
                db: &Surreal<Db>,
                owner_id: String,
            ) -> surrealdb::Result<Vec<EventRecord>> {
                // Define the query to select events for the given owner_id.
                let query = r#"
                    SELECT * FROM events WHERE owner_id = $owner_id
                "#;
        
                let mut events: Vec<SurrealEventRecord> = Vec::new();
                events = db.query("SELECT * FROM events WHERE owner_id = $owner_id")
                            .bind(("owner_id", owner_id.clone()))
                            .await
                            .unwrap()
                            .take(0)
                            .unwrap();
                let events = events.into_iter().map(SurrealEventRecord::into_event).collect();
        
                Ok(events)
            }

            pub async fn delete_event(db: &Surreal<Db>, event_id: String) -> surrealdb::Result<Response> {
                let query = r#"
                    DELETE ONLY events:$event_id;
                "#;

                let params = json!({
                    "event_id": event_id,
                });

                let response = db.query(query).bind(params).await?;
                Ok(response)
            }


        }


        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct SurrealEventRecord {
            pub id: RecordId,
            pub owner_id: RecordId,

            pub name: String,
            pub description: String,
        
            pub label: String,
        
            pub timestamp: i64
        }

        impl SurrealEventRecord {
            pub fn into_event(self) -> EventRecord {
                let id = self.id.key().to_string();
                let mut id = id.chars();
                // id.next();
                // id.next_back();

                let owner_id = self.owner_id.key().to_string();
                let mut owner_id = owner_id.chars();
                // owner_id.next();
                // owner_id.next_back();

                EventRecord {
                    id: id.as_str().to_string(),
                    owner_id: owner_id.as_str().to_string(),
                    name: self.name,
                    description: self.description,

                    label: self.label,
                    timestamp: self.timestamp
                }
            }
        }
    }
}