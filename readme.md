# Structured Query Language (SQL) Parser

## Brief description

This project implements a simple SQL `SELECT` queries parser written in Rust using the `pest` crate. It supports a subset of SQL with common features such as `SELECT`, `FROM`, `WHERE`, `JOIN`, `GROUP BY`, and basic aggregate functions like `SUM`, `AVG`, `COUNT`, etc. The goal of this parser is to convert SQL-like queries into a structured representation that can later be used for query optimization, execution, or further processing.

### Example queries

1. `SELECT name, age FROM users WHERE age > 21;`
2. `SELECT name, COUNT(*) FROM users GROUP BY name;`
3. `SELECT u.name, o.amount FROM users u JOIN orders o ON u.id = o.user_id;`


## Technical description of the parsing process

The parser uses the `pest` crate to process SQL `SELECT` queries. It follows these steps:

1. Tokenization: The input query is split into tokens based on SQL syntax rules (keywords, identifiers, operators, etc.).
2. Parsing: These tokens are matched against grammar rules to structure the query correctly.
3. Query Representation: The parsed query is transformed into a structured format for further processing, like execution or optimization.
