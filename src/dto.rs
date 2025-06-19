use diesel::QueryableByName;

#[derive(Debug, QueryableByName)]
pub struct SQLJsonResult {
    #[diesel(sql_type = diesel::sql_types::Json)]
    pub data: serde_json::Value,
}
