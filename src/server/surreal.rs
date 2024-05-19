use surrealdb::sql::Thing;

use crate::c::ID;

pub const SESSIONDB: &str = "sessions";
pub const USERDB: &str = "users";

pub fn thing_to_id(thing: &Thing) -> ID {
    thing.id.to_raw()
}

pub fn id_to_thing(id: impl Into<surrealdb::sql::Id>, tb: impl Into<String>) -> Thing {
    Thing {
        id: id.into(),
        tb: tb.into(),
    }
}
