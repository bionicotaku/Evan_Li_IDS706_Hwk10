[![CI](https://github.com/bionicotaku/Evan_Li_IDS706_Hwk10/actions/workflows/ci.yml/badge.svg)](https://github.com/bionicotaku/Evan_Li_IDS706_Hwk10/actions/workflows/ci.yml)
## Evan_Li_IDS706_Hwk10
### File Structure
```
Evan_Li_IDS706_Hwk10/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/
│       └── ci.yml
├── data/
│   ├── mydatabase.db   -> database
│   ├── gen.py          -> generate fake sample data
│   ├── requirements.txt-> python dependencies
│   ├── createTables.sql-> create tables
│   └── load.sql        -> load sample data
├── src/                
│   ├── main.rs         -> rust main function       
│   ├── lib.rs 
│   ├── crud.rs         -> CRUD operations implementation
│   └── setup.rs        -> database setup
├── tests/             
│   └── main_test.rs    -> rust test file
├── .gitignore
├── Cargo.toml          -> rust dependencies
├── Cargo.lock
├── Makefile
├── README.md
└── LICENSE
```
## Intrduction

This is a Rust-based SQLite database management project that demonstrates CRUD (Create, Read, Update, Delete) operations. The project features:

1. Used a python script to generate fake data with the `faker` library
2. Used sql language to create a database with 3 tables and load the fake data
3. Encapsulated these basic CRUD database operations into functions using `rusqlite` library, allowing users to directly call API functions based on business logic without having to write SQL statements
4. Sample test cases are provided in the main function

## Links
1. [Video Demo](https://youtu.be/N383KLntjs0)
2. [Github Actions](https://github.com/bionicotaku/Evan_Li_IDS706_Hwk10/actions/workflows/ci.yml)

## Preparation
1. First of all, install the python packages and update rust toolchain `make install`
2. Format code `make format`
3. Lint code `make lint`
4. Test code `make test`
5. Build rust code `make build`
6. Build the release version `make release`

## Run The Code
`make run`
The program will generate fake data, create the database and load the data. Then it will call the CRUD functions to implement some sample test cases. All the process will be printed.

## Use of LLMs
Since I already have a similar Python project, I only need to refactor it using Rust. After redesigning the code structure and defining the input/output interfaces, I broke down the code into functions and asked Claude to help with translation and refactoring.
Due to the limited corpus of Rust code, Claude's generated code had many issues and wasn't directly usable. I mainly used the generated code to learn and understand Rust syntax and library usage, and carefully modified it myself.
After completing the program, I had Claude help write the main_test.rs file for testing. Many bugs emerged during this process, but I eventually debugged and fixed the errors based on the error messages to get the correct test results.


## Database Schema

This database consists of three main tables: Users, Products, and Orders.

### Users Table

Stores information about users.

| Column    | Type    | Constraints                |
|-----------|---------|----------------------------|
| user_id   | INTEGER | PRIMARY KEY, AUTOINCREMENT |
| email     | TEXT    | UNIQUE, NOT NULL           |
| firstname | TEXT    | NOT NULL                   |
| lastname  | TEXT    | NOT NULL                   |
| address   | TEXT    | NOT NULL                   |
| balance   | REAL    | DEFAULT 0, NOT NULL        |

### Products Table

Contains details about available products.

| Column       | Type    | Constraints                |
|--------------|---------|----------------------------|
| product_id   | INTEGER | PRIMARY KEY, AUTOINCREMENT |
| product_name | TEXT    | NOT NULL                   |
| category     | TEXT    | NOT NULL                   |
| price        | REAL    | NOT NULL, CHECK (price >= 0) |
| stock        | INTEGER | NOT NULL, CHECK (stock >= 0) |

### Orders Table

Tracks orders made by users.

| Column     | Type    | Constraints                |
|------------|---------|----------------------------|
| order_id   | INTEGER | PRIMARY KEY, AUTOINCREMENT |
| user_id    | INTEGER | NOT NULL, FOREIGN KEY      |
| product_id | INTEGER | NOT NULL, FOREIGN KEY      |
| quantity   | INTEGER | NOT NULL, CHECK (quantity > 0) |
| order_date | TEXT    | NOT NULL                   |
