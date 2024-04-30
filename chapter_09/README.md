# Chapter 8: API Integration

## Postgres and Docker Compose

- The PostgreSQL database is used to store the questions and answers. The database is started using Docker Compose.
- The Docker Compose configuration file contains an initialization script, `rest_server/db/init.sql`, which is used
  to create the role and database and transfer ownership of the database.
    - Use `docker-compose up` to start the PostgreSQL database.
    - Use `docker-compose down --volumes` to stop the PostgreSQL database.
- The database is available at `localhost:5432` with the username `rustwebdev` and password `rustwebdev`.
- The database name is `rustwebdev_db`.

## Environment Variables

- Using the `dotenv` crate to load environment variables from a `.env` file.
- The `.env` file contains the following environment variables:
    - `BAD_WORDS_API_KEY`: The API key from apilayer.com. Get a free API key from https://apilayer.com/.
    - `POSTGRES_CONNECTION_STRING`: The Postgres connection string.

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
      "title": "The second question ever asked for this service was bullshit!",
      "content": "This is the second question content ever asked for this service was bullshit!"
}'
```

### Update a question by id

```shell
curl --location --request PUT 'localhost:3030/questions/1' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "id": 1,
      "title": "Second question ever asked for this service. Fuck!",
      "content": "How on Mars did I get this to work? Fuck!!!"
}'
```

### Delete a question by id

```shell
curl --location --request DELETE 'localhost:3030/questions/1' 
```