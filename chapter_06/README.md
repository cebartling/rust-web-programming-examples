# Chapter 6: Logging and Tracing

## Logging

- Use `RUST_LOG=info` to set the log level to info. For example, `RUST_LOG=info cargo run`.
- Use `RUST_LOG=debug` to set the log level to debug. For example, `RUST_LOG=debug cargo run`.


## Tracing


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
      "id": "10",
      "title": "question title",
      "content": "question content"
}'
```

### Update a question by id
```shell
curl --location --request PUT 'localhost:3030/questions/QI0001' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "id": "QI0001",
      "title": "First question ever asked for this service",
      "content": "How on Earth did I get this to work?"
}'
```

### Delete a question by id
```shell
curl --location --request DELETE 'localhost:3030/questions/QI0001' 
```