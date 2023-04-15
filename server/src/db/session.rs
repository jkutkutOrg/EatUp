use super::*;

pub async fn get_sessions(
    db: &State<Client>,
    filters: SessionQuery
) -> Result<Vec<Session>, Status> {
    let mut query: String = "SELECT * FROM session".to_string();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut count = 1;

    if filters.table_ids.len() > 0 {
        query.push_str(format!(" WHERE table_id = ANY(${})", count).as_str());
        params.push(&filters.table_ids);
        count += 1;
    }
    if filters.in_progress.is_some() {
        if count > 1 {
            query.push_str(" and ");
        } else {
            query.push_str(" WHERE ");
        }
        query.push_str(format!(" in_progress = ${}", count).as_str());
        params.push(&filters.in_progress);
    }

    let mut sessions: Vec<Session> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &params).await.unwrap() {
        sessions.push(Session::new(
            row.get(0), // id
            row.get(1), // table_id
            row.get(2) // in_progress
        ));
    }
    Ok(sessions)
}

pub async fn create_session(
    db: &State<Client>,
    table_id: String
) -> Result<SessionUuid, InvalidAPI> {
    let query: String = "SELECT * FROM create_session($1)".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    match db.query_one(&stmt, &[&table_id]).await {
        Ok(row) => {
            let simple_id: String = row.get(0);
            let id: Uuid = row.get(1);
            let id_str = id.to_string();

            let qr_path = format!("/qr/{}.png", &id_str);
            let qr_real_path = format!("/db/public{}", &qr_path);

            qr::generate_with_debug(&id_str, &qr_real_path);
            Ok(SessionUuid::new(
                simple_id,
                id,
                qr_path // qr_img
            ))
        },
        Err(_) => Err(InvalidAPI::new(
            format!("There is already a session in progress for table {table_id}")
        ))
    }
}

pub async fn end_session(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<(), Status> {
    let session_id: Uuid = session_id.unwrap();
    let query: String = "UPDATE session SET in_progress = false WHERE id = $1;".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    db.execute(&stmt, &[&session_id]).await.unwrap();
    let file = format!("/db/public/qr/{}.png", session_id.to_string());
    std::fs::remove_file(file).unwrap();
    Ok(())
}