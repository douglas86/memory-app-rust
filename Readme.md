# Table of Content

- [Setting up iced](#setting-up-iced)
  [Setting up Axum](#setting-up-axum)
  [Setting up Surreal DB](#setting-up-surreal-db)

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

## [Setting up Axum](#table-of-content)

This creates a new workspace called server

```bash
cargo new server
```

Add Axum and Tokio packages to the cargo.toml file

```bash
cargo add axum -p server
cargo add tokio --features full -p server
```

[Tutorial](https://www.rustfinity.com/blog/axum-rust-tutorial) to get a simple get and post request for a hello world
program

create a file call test.http this is where all the tests sit for the http requests

---

## [Setting up surreal DB](#table-of-content)

### Installing Podman

Development Setup: Podman on Windows

### 1. Installations

Install the official Podman CLI toolchain via Windows Package Manager:

```powershell
winget install Redhat.Podman
```

Note: If Podman is not recognized immediately after installation, ensure is added to Path

```powershell
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";C:\Program Files\RedHat\Podman", "User")
```

Restart your terminal session after updating environment variables.

Run the following commands to download and install as well as test podman

```powershell
podman machine init
podman machine start
podman machine list
```

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