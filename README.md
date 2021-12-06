What's this?

A repo with some experimentation with actix, tokio, postgres..

Refs:
- https://medium.com/@ilegra/building-a-microservice-with-rust-ef9641cf2331
- https://medium.com/tenable-techblog/building-a-microservice-with-rust-23a4de6e5e14

The initial commit in this repo was with an exact copy of
[rust-playground/rust-microservice](
  https://github.com/diegopacheco/rust-playground/tree/4bf783410c6dc112212564aae32701889c79bc12/rust-microservice
) (thanks Diego for the cool resources).

Then some readme and code adjustments (including `cargo fmt`), etc.

So far, all very basic but running fine.

As time permits I may be adding things like
[diesel](https://diesel.rs/),
[juniper](https://github.com/graphql-rust/juniper),
...

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
just db-create
```

### Run service

```bash
just run-service
```
### Test it

In a separate terminal:

```bash
just db-news
just db-news-add-some
just db-news <id>
just db-news-all
just db-news-delete <id>
just db-news-delete-all
```
