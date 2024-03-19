## Web Service Tutorial in Rust

## Overview

This project serves as a comprehensive tutorial for building a web service using the Rust programming language. It covers various aspects, from setting up a PostgreSQL database to handling JSON responses.

## Technologies Used

- **Rust**: The core programming language used for this project.
- **Tokio**: An asynchronous runtime for Rust.
- **SQLx**: A Rust library for connecting to PostgreSQL databases.
- **Dotenv**: A Rust crate for handling `.env` files.
- **Serde**: A Rust crate for serializing and deserializing data structures.
- **Actix-Web**: A Rust framework for building web services.
- **Chrono**: A time manipulation library for Rust.

## Documentation

To generate documentation for this project:

```bash
cargo doc --open
```

## Getting Started

### Prerequisites

- Ensure you have Docker Compose installed. If not, you can download it from [Docker Desktop](https://www.docker.com/products/docker-desktop).

### Setup

1. **Environment Variables**: Copy the sample environment file and configure the variables.

    ```bash
    cp env.example .env
    ```

2. **Docker Compose**: Start the Docker containers.

    ```bash
    docker compose up -d
    ```
3. **Upload Postman Collection**: Open up Postman locally via the desktop client. Then upload the file entitled `Web Service Tutorial.postman_collection.json`. You should now have    access to the Postman collection.
4. **Get a Bearer Token**:
    1. Go to the `create_user` route under the `auth` folder.
    2. Create a user by hitting the `Send` button. If there's a problem, change both the username and password.
    3. After you have successfully created a new user, check the headers. Under the `authorization` header, you should see the bearer token. Copy everything after where it says `Bearer`.
    4. Now click on the `Web Service Tutorial` Postman collection.
    5. Under the `Authorization` page, you should see a `Type` dropdown.
    6. Select `Bearer Token`. After you select `Bearer Token`, underneath should apear another dropdown that says `Token`. Paste the copied bearer token here from before. You should now be authorized for four hours, or for however long `JWT_HOURS_ACTIVE` is set to in your .env file.
    7. If you have any issues hitting the routes, try changing every `localhost:8080` to `http://127.0.0.1:8080`

5. **Test Routes**: Open your browser or use a tool like Postman to hit the following route:

    ```
    http://127.0.0.1:8080/api/v1/blog
    ```

### Database GUI (PgAdmin4)

- Access the PgAdmin4 interface at `http://localhost:16543`.
- Username: `test@test.com`
- Password: `test`

For detailed instructions on adding a PostgreSQL server in PgAdmin4, refer to the [PG Admin guide](https://onexlab-io.medium.com/docker-compose-postgres-initdb-ba0021deef76).

### SQL Schema

The `init.sql` file contains the SQL statements that define the database schema. Feel free to explore it to understand the database structure.

### Authentication

The `SKIP_AUTH` environment variable controls JWT authentication. Set it to `true` to disable JWT during development.

## Additional Resources

- [Crates.io Package](https://crates.io/crates/webdev_guide)
- [PG Admin Guide](https://onexlab-io.medium.com/docker-compose-postgres-initdb-ba0021deef76)
- Medium Post: [Creating a Web Service in Rust with Actix-Web, SQLx, and PostgreSQL](https://medium.com/@elijahobara/building-a-web-service-in-rust-with-actix-web-sqlx-and-postgresql-a70816c07b9c)