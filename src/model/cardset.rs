use leptos::{prelude::*, server, server_fn::ServerFn};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct CardsetRecord {
    pub id: String,
    pub owner_id: String,

    pub name: String,
    pub cards: Vec<Flashcard>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flashcard {
    pub question: String,
    pub answer: String
}

impl CardsetRecord {}

#[server(GetCardsets, "/api")]
pub async fn get_cardsets() -> Result<Vec<CardsetRecord>, ServerFnError> {
    use self::*;
    use crate::model::*;
    use crate::auth::*;
    let db = db()?;
    let user = auth()?;

    Ok(CardsetRecord::get_cardsets_for_user(&db, user.id).await?)
}

#[server(CreateCardset, "/api")]
pub async fn create_cardset(name: String) -> Result<(), ServerFnError> {
    use self::*;
    use crate::model::*;
    use crate::auth::*;

    let db = db()?;
    let user = auth()?;
    CardsetRecord::create_cardset(&db, user.id, name).await?;
    Ok(())
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use surrealdb::{Response, Value};
        use surrealdb::{engine::local::Db, RecordId, Surreal};

        impl CardsetRecord {
            pub async fn create_cardset_table(db: &Surreal<Db>) {
                db.query(
                    "   DEFINE TABLE cardsets SCHEMAFULL; 
                        DEFINE FIELD owner_id ON TABLE cardsets TYPE record<students>;
                        DEFINE FIELD name ON TABLE events TYPE string;
                        DEFINE FIELD cards ON cardsets TYPE array<object>;
                        DEFINE FIELD cards[*] ON cardsets TYPE object;
                    ",
                    );
            }

            pub async fn get_cardset(db: &Surreal<Db>, cardset_id: String) -> Option<CardsetRecord> {
                let cardset: Option<SurrealCardsetRecord> = db
                .query("SELECT id, owner_id, cards FROM cards where id = $id")
                .bind(("id", cardset_id))
                .await
                .unwrap()
                .take(0)
                .unwrap();

                if let Some(set) = cardset {
                    return Some(set.into_cardset());
                }
                
                None
            }

            pub async fn create_cardset(db: &Surreal<Db>, owner_id: String, name: String) -> surrealdb::Result<Response> {
                let query = r#"
                    CREATE events CONTENT {
                        owner_id: $owner_id,
                        name: $name,
                        cards: []
                    }
                "#;

                let params = json!({
                    "owner_id": owner_id,
                    "name": name,
                });

                let response = db.query(query).bind(params).await?;

                Ok(response)
            }

            pub async fn get_cardsets_for_user(
                db: &Surreal<Db>,
                owner_id: String,
            ) -> surrealdb::Result<Vec<CardsetRecord>> {
                // Define the query to select events for the given owner_id.
                let query = r#"
                    SELECT * FROM cardsets WHERE owner_id = $owner_id
                "#;
        
                let mut events: Vec<SurrealCardsetRecord> = Vec::new();
                events = db.query("SELECT * FROM events WHERE owner_id = $owner_id")
                            .bind(("owner_id", owner_id.clone()))
                            .await
                            .unwrap()
                            .take(0)
                            .unwrap();
                let events = events.into_iter().map(SurrealCardsetRecord::into_cardset).collect();
        
                Ok(events)
            }
        }

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct SurrealCardsetRecord {
            pub id: RecordId,
            pub owner_id: RecordId,
            pub name: String,
            pub cards: Vec<Flashcard>,
        }
        impl SurrealCardsetRecord {
            pub fn into_cardset(self) -> CardsetRecord {
                let id = self.id.key().to_string();
                let mut id = id.chars();
                // id.next();
                // id.next_back();

                let owner_id = self.owner_id.key().to_string();
                let mut owner_id = owner_id.chars();
                // owner_id.next();
                // owner_id.next_back();

                CardsetRecord {
                    id: id.as_str().to_string(),
                    owner_id:  owner_id.as_str().to_string(),
                    name: self.name,
                    cards: self.cards
                }
            }
        }

}}
