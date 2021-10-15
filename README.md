What's this?

A repo with some experimentation with actix, tokio, postgres..

Refs:
- https://medium.com/@ilegra/building-a-microservice-with-rust-ef9641cf2331
- https://github.com/diegopacheco/rust-playground/tree/master/rust-microservice

The initial commit in this repo was with an exact copy of
[rust-playground/rust-microservice](
  https://github.com/diegopacheco/rust-playground/tree/4bf783410c6dc112212564aae32701889c79bc12/rust-microservice
).

Then some readme and code adjustments (including `cargo fmt`), etc.

### Build

```bash
cargo build
```

### Database setup

In a separate terminal:

```bash
just run-postgres
```

Create database:

```bash
just create-database
```

### Run service

```bash
just run-service
```
### Test it

In a separate terminal:

```bash
curlie http://localhost:8080/news
curlie put "http://localhost:8080/news/facebook/faceboo.com"
curlie http://localhost:8080/news/863282a8-62f8-06b3-eb55-fbc44e444a0b
curlie delete "http://localhost:8080/news/0dae39e4-fca2-b076-4f88-617dd3352d11"
```
