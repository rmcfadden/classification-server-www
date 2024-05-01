#[cfg(test)]
mod tests {
    use classification_server_www::models::sqlite_model_store::SqliteModelStore;
    use classification_server_www::models::model_store::ModelStore;
    #[async_std::test]
    async fn test_sqlite_store() {
        let store = SqliteModelStore::<String> { test: "asdf".to_string() };
        store.setup().await.unwrap();
    }
}
