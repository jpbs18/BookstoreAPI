# Rust API Backend

This is a simple API backend built with Rust that provides user authentication and manages books and authors. The application uses [SeaORM](https://www.sea-orm.org/) for database interactions and [MySQL](https://www.mysql.com/) as the database management system.

## Features

- User authentication with secure login and registration.
- Management of books and authors.
- Built using Rust, offering performance and safety.

## Technologies Used

- **Rust**: A systems programming language that is fast and memory-efficient.
- **SeaORM**: An async & dynamic ORM for Rust, facilitating easy database interactions.
- **MySQL**: A relational database management system for storing data.

## Getting Started

### Prerequisites

Make sure you have the following installed on your machine:

- Rust (install from [rustup.rs](https://rustup.rs/))
- MySQL server (installation instructions can be found [here](https://dev.mysql.com/doc/refman/8.0/en/installing.html))

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/your-repo-name.git
   cd your-repo-name

2. Create a .env file in the root directory and set up your database connection parameters:

   BOOKSTORE_DB_HOST=host
   BOOKSTORE_DB_PORT=port
   BOOKSTORE_DB_USERNAME=user
   BOOKSTORE_DB_PASSWORD=password
   BOOKSTORE_DB_DATABASE=database
   BOOKSTORE_JWT_SECRET=secret

3. Run the following command to build and run the application:

   cargo run

4. The API will be available at http://localhost:8000

#API Endpoints

##Authentication
POST /api/auth/register: Register a new user.
POST /api/auth/login: Log in an existing user.

Books
GET /api/books: Retrieve a list of all books.
POST /api/books: Add a new book.
GET /api/books/{id}: Retrieve a specific book by ID.
PUT /api/books/{id}: Update a specific book by ID.
DELETE /api/books/{id}: Delete a specific book by ID.

Authors
GET /api/authors: Retrieve a list of all authors.
POST /api/authors: Add a new author.
GET /api/authors/{id}: Retrieve a specific author by ID.
PUT /api/authors/{id}: Update a specific author by ID.
DELETE /api/authors/{id}: Delete a specific author by ID.
