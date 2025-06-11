use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).expect("Failed to create dogs table");

        conn
    };
}

#[server]
pub async fn save_dog(image: String) -> Result<usize, ServerFnError> {
    let id = DB.with(|conn| -> Result<usize, rusqlite::Error> {
        conn.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image])?;
        Ok(conn.last_insert_rowid() as usize)
    })?;

    Ok(id)
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|conn| -> Result<Vec<(usize, String)>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 20")?;
        let dogs = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(dogs)
    })?;

    Ok(dogs)
}

#[server]
pub async fn delete_dog(id: usize) -> Result<(), ServerFnError> {
    println!("Attempting to delete dog with ID: {}", id);

    let rows_affected = DB.with(|conn| -> Result<usize, rusqlite::Error> {
        conn.execute("DELETE FROM dogs WHERE id = ?1", &[&id])
    })?;

    if rows_affected == 0 {
        return Err(ServerFnError::ServerError(format!(
            "No dog found with ID {}",
            id
        )));
    }

    println!("Rows affected: {}", rows_affected);
    Ok(())
}
