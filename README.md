# Rust backend with Axum, SurrealDB and Email Verification

## 🛠️ Features

- **User Authentication**: Register, login, password reset functionality.
- **Email Verification**: Users receive an email to verify their accounts.
- **SurrealDB Integration**: Store and manage user data securely.
- **JWT Authentication**: Secure API endpoints with JSON Web Tokens (JWT).
- **Middleware**: Implement custom middleware for authentication.
- **Testing with Postman**: A full Postman collection is provided to test all API endpoints.

## 🚀 Getting Started

### Prerequisites

To run this project, you will need:

- [Rust](https://www.rust-lang.org) installed on your machine.
- [SurrealDB](https://surrealdb.com) installed and running locally or remotely.
- [Postman](https://www.postman.com/) for testing API endpoints.

### Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/DarcyJason/axum_surrealdb_template.git
    cd axum_surrealdb_template
    ```

2. **install dependencies**

    ```bash
    cargo install --path .
    ```

3. **Set up SurrealDB** (Recommend to use [Surrealist](https://surrealdb.com/docs/surrealist/installation)(SurrealDB GUI APP) to start SurrealDB server, you can change the settings in the Surrealist to fit the following settings)

    ```bash
    surreal start rocksdb:~/surrealdb -u root -p root -b 0.0.0.0:10086
    ```

4. **Init SurrealDB**

    If you use Surrealist, you can just click to create namespace `web_app` and database `backend`.

    ```bash
    echo "DEFINE NS web_app; USE NS web_app; DEFINE DB backend;" | surreal sql --conn http://localhost:10086 -u root -p root
    ```

5. **Configure .nev file**

    If you want to change the server port or change the frontend url, you can configure it on .env file.

6. **Start the server**

    ```bash
    cargo run
    ```

    The server will be running on `http://localhost:7878`.
