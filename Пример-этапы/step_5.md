# Адаптер для работы с БД.

Подключение необходимых библиотек:

Следующие библиотеки будут нужны при дальнейшей разработке сервиса.

## Библиотека для асинхронного выполнения функций.

`async` `await` - ключевые слова для работы с асинхронными функциями в `Rust`. Асинхронное выполнение позволяет не блокировать (останавливать) основной поток выполнения программы (перекладывать задачу на другой поток). Таким образом программа не зависает. Взаимодействие с базой данных или другими внешними (`IO-операции`) ресурсами выполняется асинхронно.

```toml
[dependencies.smol]
version = "2"
```

```toml
[dependencies.rusqlite]
version = "0.40.2"
features = ["bundled", "functions", "time", "uuid"]
```

```toml
[dependencies.async-sqlite]
version = "0.6.0"
features = ["array", "backup", "blob", "bundled", "cache", "time", "uuid", "vtab", "window"]
```

```toml
[dependencies.async-lock]
version = "3.4.2"
```

# Создание трейтов-адаптеров. 

Трейты - это структуры, которые описывают, что будут "некоторые действия", которые "кем-то будут выполнятся", но конкретно кем они будут выполнятся: неизвестно. По-другому говоря, они описывают контракт, который кто-то будет реализовывать в коде. Например кто-то будет реализовывать функции для работы с базой данных. **Трейты всегда реализуются структурами или перечислениями.**

1. Создать папку `contracts`.

2. Объявить модуль `contracts` (создать `mod.rs`).

3. Объявить трейты для сущностей (например есть сущностЬ `contract.rs`), её я собираюсь хранить в БД:

```rust
pub trait ContractsStorage {
    
}
```

```rust
pub trait ContractsStorage {
    fn add(&self, contract: Contract) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn remove(&self, contract: Contract) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(&self, uuid: Uuid) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(&self contract: &Contract) -> 
        impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
```

# Реализация трейтов-адаптеров.

Реализация трейта - значит, что есть структура или перечисление, которое будет выполнять действия, описанные в трейте. Иными словами, структура или перечисление берёт на себя обязательство реализовать все функции, объявленные в трейте.

1. Создать папку `contracts_impl`.

2. Объявить папку `contracts_impl` модулем (создать `mod.rs`).

3. Объявите структуру, которая будет в дальнейшем реализовывать трейт `ContractsStorage`.

```rust
pub struct ContractsSqliteStorage {

}
```

Работа с базой данных в библиотеке `async-sqlite` осуществляется при помощи пула соединений (`Pool`). Пул соединений содержит набор активных соединений с базой данных `sqlite`, которые могут использоваться одновременно для выполнения асинхронных запросов.

```rust
use async_sqlite::Pool;

pub struct ContractsSqliteStorage {
    connection: Pool,
}
```

4. Для работы с `sqlite` необходимо создать пул соединений, указав путь к базе данных `sqlite`:

Следующий код позволяет создать пул соединений с базой данных `sqlite` и инициализировать структуру `ContractsSqliteStorage`.

```rust
impl ContractsSqliteStorage {
    pub fn new(database_url: &str) -> Self {
        let path_buf: PathBuf = PathBuf::from(database_url);
        let builder: PoolBuilder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);
        let connection: Pool = builder.open();
        Self { connection }
    }
}
```

_Но он не работает,_ потому что `builder.open()` - это асинхронная функция, и её нужно вызывать с `await` внутри асинхронного контекста.

Чтобы вызывать `await`, необходимо, чтобы вызывающий блок кода так же был асинхронный:

```rust
pub async fn new(database_url: &str) -> Self {
    let path_buf: PathBuf = PathBuf::from(database_url);
    let builder: PoolBuilder = PoolBuilder::new()
        .path(path_buf)
        .journal_mode(async_sqlite::JournalMode::Wal)
        .num_conns(10);
    let connection: Pool = builder.open().await.expect("Database creation error");
    Self { connection }
}
```

Следующий код - временный, он нужен, чтобы создать соединение через `await` и сразу получить объект `Pool`. Он может уронить программу, если не удастся создать соединение, например, если путь к `sqlite` базе неправильный.

```rust
let connection: Pool = builder.open().await.expect("Database creation error");
```

5. Импортируйте и реализуйте трейт, который создавали раннее для работы с контрактами (`ContractsStorage`).

```rust
impl ContractsStorage for ContractsSqliteStorage {

    async fn add(&self, contract: crate::entities::Contract) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get(&self, uuid: uuid::Uuid) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn remove(&self, contract: crate::entities::Contract) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn update(&self, contract: &crate::entities::Contract) -> 
        Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

}
```

6. Теперь можно каждую конкретную операцию: `add`, `get`, `remove` и `update` реализовать с использованием соединения с базой данных `sqlite`. Здесь потребуются знания написания SQL запросов:

## `add` - добавление записи.

`self.connection.conn(...)` - это функция пула `Pool`, которая "забирает" одно свободное соединение из пула и выполняет внутри переданного замыкания (`closure`) обычный синхронный код `rusqlite`. Именно поэтому внутри `conn` нет `await` - там выполняется синхронный код, а `await` стоит только снаружи, потому что сама операция получения соединения из пула - асинхронная.

Замыкание помечено `move`, чтобы оно забрало владение переменной `parameters` (и всеми данными, которые понадобятся внутри) - это нужно, потому что замыкание может быть выполнено в другом потоке, а `Rust` не разрешает просто так "одалживать" переменные между потоками.

```rust
async fn add(
    &self,
    contract: crate::entities::Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    const QUERY: &str = "
        INSERT INTO contracts (id, name, created_at)
        VALUES (?1, ?2, ?3)
    ";

    let mut parameters: Vec<Value> = Vec::with_capacity(3);
    parameters.push(contract.id.into());
    parameters.push(contract.customer_id.into());
    parameters.push(contract.created_at.to_string().into());

    self.connection
        .conn(move |connection| {
            let affected = connection.execute(QUERY, params_from_iter(parameters))?;
            Ok(affected)
        })
        .await?;

    Ok(())
}
```

`params_from_iter(parameters)` - функция, которая превращает набор значений (`Vec<Value>`) в параметры SQL запроса. Каждое значение из вектора подставляется на место соответствующего `?1`, `?2`, `?3` по порядку. `Value` - это перечисление из `rusqlite`, которое умеет хранить в себе любой тип, поддерживаемый `sqlite` (числа, строки, бинарные данные и т.д.). Поэтому `contract.id.into()` превращает `Uuid` в `Value`, а `contract.created_at.to_string().into()` - строку в `Value`.

## `get` - получение записи по идентификатору.

Для получения одной записи используется функция `connection.query_row`. Она выполняет `SELECT` запрос и ожидает **ровно одну** строку в ответ - если строк не будет или будет больше одной, вернётся ошибка. Второй параметр - параметры запроса (как и в `execute`), третий параметр - замыкание, которое описывает, как из строки (`row`) достать нужные поля через `row.get(индекс_колонки)`.

```rust
async fn get(&self, uuid: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
    const QUERY: &str = "
        SELECT id, customer_id, created_at, finished_at
        FROM contracts
        WHERE id = ?1
    ";

    let parameters: Vec<Value> = vec![uuid.into()];

    self.connection
        .conn(move |connection| {
            connection.query_row(QUERY, params_from_iter(parameters), |row| {
                let id: uuid::Uuid = row.get(0)?;
                let customer_id: uuid::Uuid = row.get(1)?;
                let created_at: String = row.get(2)?;
                let finished_at: Option<String> = row.get(3)?;
                Ok((id, customer_id, created_at, finished_at))
            })
        })
        .await?;

    Ok(())
}
```

Обратите внимание: индексы колонок в `row.get(...)` идут в том же порядке, в каком колонки перечислены в `SELECT`. `finished_at` объявлен как `Option<String>`, потому что в таблице это поле может быть `NULL` (договор ещё не завершён) - `rusqlite` сам умеет превращать `NULL` в `None`.

_Для упрощения материала_ здесь функция просто читает строку и ничего не возвращает наружу (сигнатура трейта осталась `Result<()>`) - на практике сюда нужно было бы собрать полноценную сущность `Contract` и вернуть её вызывающему коду.

## `remove` - удаление записи.

Здесь используется тот же `execute`, что и в `add`, но с запросом `DELETE`. Так как для удаления нужен только идентификатор договора, из всей структуры `Contract` используется единственное поле - `id`.

```rust
async fn remove(
    &self,
    contract: crate::entities::Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    const QUERY: &str = "DELETE FROM contracts WHERE id = ?1";

    let parameters: Vec<Value> = vec![contract.id.into()];

    self.connection
        .conn(move |connection| {
            let affected = connection.execute(QUERY, params_from_iter(parameters))?;
            Ok(affected)
        })
        .await?;

    Ok(())
}
```

## `update` - изменение существующей записи.

Запрос `UPDATE` перезаписывает все изменяемые поля договора (`customer_id`, `created_at`, `finished_at`) по его `id`. Так как функция принимает `contract` по ссылке (`&Contract`), а не по значению, значения полей копируются/клонируются в вектор `parameters` до того, как замыкание заберёт этот вектор себе (`move`) - саму ссылку на `contract` в другой поток передавать было бы нельзя.

```rust
async fn update(
    &self,
    contract: &crate::entities::Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    const QUERY: &str = "
        UPDATE contracts
        SET customer_id = ?1, created_at = ?2, finished_at = ?3
        WHERE id = ?4
    ";

    let mut parameters: Vec<Value> = Vec::with_capacity(4);
    parameters.push(contract.customer_id.into());
    parameters.push(contract.created_at.to_string().into());
    parameters.push(contract.finished_at.map(|dt| dt.to_string()).into());
    parameters.push(contract.id.into());

    self.connection
        .conn(move |connection| {
            let affected = connection.execute(QUERY, params_from_iter(parameters))?;
            Ok(affected)
        })
        .await?;

    Ok(())
}
```

`contract.finished_at.map(|dt| dt.to_string())` превращает `Option<UtcDateTime>` в `Option<String>` - если дата есть, она конвертируется в строку, если нет (`None`) - остаётся `None`. `rusqlite` умеет превращать `Option<T>` в `Value`: `Some(значение)` подставляется как обычное значение, а `None` - как `NULL`.
