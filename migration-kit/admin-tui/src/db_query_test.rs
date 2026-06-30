use std::path::Path;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = Path::new("c:/Users/dmene/Projetos/innovai/git/CLI_elevve_clinic/migration-kit/sql/elevve_clinic_deva_db.db");
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT id, full_name, role FROM deva_elevveclinic_users LIMIT 5;")?;
    let user_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    })?;
    println!("id | full_name | role");
    for user in user_iter {
        let (id, full_name, role) = user?;
        println!("{} | {} | {}", id, full_name, role);
    }
    Ok(())
}
