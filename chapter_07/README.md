# Chapter 7: Integration with PostgreSQL

## Postgres and Docker Compose

- The PostgreSQL database is used to store the questions and answers. The database is started using Docker Compose.
- The Docker Compose configuration file contains an initialization script, `rest_server/db/init.sql`, which is used
  to create the role and database and transfer ownership of the database.
    - Use `docker-compose up` to start the PostgreSQL database.
    - Use `docker-compose down --volumes` to stop the PostgreSQL database.
- The database is available at `localhost:5432` with the username `rustwebdev` and password `rustwebdev`.
- The database name is `rustwebdev_db`.

## sqlx

- The `sqlx` crate is used to interact with the PostgreSQL database.
- The `migrations` directory contains the SQL migration files. When the web server starts, the migration files are
  executed by sqlx:

  ```rust
  let store =
      store::Store::new("postgres://localhost:5432?dbname=rustwebdev_db&user=rustwebdev&password=rustwebdev").await;
  
  sqlx::migrate!()
      .run(&store.clone().connection)
      .await
      .expect("Cannot migrate DB");
  ```

## Logging

- Use `RUST_LOG=info` to set the log level to info. For example, `RUST_LOG=info cargo run`.
- Use `RUST_LOG=debug` to set the log level to debug. For example, `RUST_LOG=debug cargo run`.

## Acceptance Testing

### Get all questions

```shell
curl --location --request GET 'localhost:3030/questions'
```

### Create a new question

```shell
curl --location --request POST 'localhost:3030/questions' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "title": "The second question ever asked for this service",
      "content": "This is the second question content ever asked for this service"
}'
```

### Update a question by id

```shell
curl --location --request PUT 'localhost:3030/questions/2' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "id": 2,
      "title": "Second question ever asked for this service",
      "content": "How on Mars did I get this to work?"
}'
```

### Delete a question by id

```shell
curl --location --request DELETE 'localhost:3030/questions/QI0001' 
```