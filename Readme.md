# Table of Content

- [Setting up iced](#setting-up-iced)

1. [Project Structure](#project-structure)
2. [Credits](#credits)

---

## [Setting up iced](#table-of-content)

This creates a new workspace called client

```bash
cargo new client
```

This adds the iced toolkit to the client workspace

```bash
cargo add iced -p client
```

[Tutorial](https://book.iced.rs/first-steps.html) to get a simple increment and decrement button to work with the iced
toolkit

---

## [Project Structure](#table-of-content)

```text
|-- Cargo.toml
|-- src
    |-- main.rs
|-- client/
    |-- main.rs # Runs the main iced interface with increment and decrement buttons
|-- server/
    |-- src/
        |-- main.rs # Runs a simple get and post request from the home route
    |-- test.http # Test Get and Post route from home path
|-- add_one/
```

---

## [Credits](#table-of-content)

- [Creating a Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

---