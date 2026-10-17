use async_sqlite::{Client, ClientBuilder, JournalMode};

/// Управляемый пул соединений: набор из отправителя и получателя канала,
/// через которые соединения выделяются и возвращаются обратно.
type ManagedPool = (
    smol::channel::Sender<Client>,
    smol::channel::Receiver<Client>,
);

/// Пул соединений с базой данных `SQLite`, который гарантирует,
/// что в рамках одной бизнес-операции будет использоваться
/// одно и то же выделенное соединение.
pub struct ManagedConnectionPool {
    database_path: String,
    max_connections_count: usize,
    pool: ManagedPool,
}

impl ManagedConnectionPool {
    /// Создаёт пул и сразу наполняет его соединениями
    /// в количестве `max_connections_count`.
    pub async fn initialize(database_path: String, max_connections_count: usize) -> Self {
        let path_clone = database_path.clone();
        let (sender, receiver) = smol::channel::bounded::<Client>(max_connections_count);
        for _ in 0..max_connections_count {
            let connection = Self::create_connection(path_clone.clone()).await;
            sender
                .send(connection)
                .await
                .expect("Не удается наполнить пул соединений.")
        }

        Self {
            database_path: path_clone,
            max_connections_count,
            pool: (sender, receiver),
        }
    }

    /// Открывает новое одиночное соединение с базой данных.
    async fn create_connection(database_path: String) -> Client {
        let builder: ClientBuilder = ClientBuilder::new()
            .path(database_path.clone())
            .journal_mode(JournalMode::Wal);
        builder.open().await.expect("Не удается открыть соединение.")
    }

    /// Забирает одно свободное соединение из пула.
    pub async fn acquire_connection(&self) -> Result<Client, Box<dyn std::error::Error>> {
        let connection = self.pool.1.recv().await.map_err(|error| {
            format!(
                "Не удалось получить соединение из пула ({}): {error}",
                self.database_path
            )
        })?;
        Ok(connection)
    }

    /// Возвращает соединение обратно в пул после использования.
    pub async fn return_connection(
        &self,
        connection: Client,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.pool.0.send(connection).await.map_err(|error| {
            format!(
                "Не удалось вернуть соединение в пул ({}), размер пула {}: {error}",
                self.database_path, self.max_connections_count
            )
        })?;
        Ok(())
    }
}
