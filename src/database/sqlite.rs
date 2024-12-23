use diesel::prelude::*;

use super::DbResult;

fn establish_connection() -> DbResult<SqliteConnection> {
    let conn = SqliteConnection::establish(&crate::ENV.lock().unwrap().conn_str)?;
    Ok(conn)
}
