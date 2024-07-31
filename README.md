# TODO App

This TODO application is built using Axum and Shuttle.rs for the back-end, with a lightweight front-end using Pico.css for styling and Alpine.js for interactions.

## Prerequisites

Make sure you have the following installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Shuttle](https://www.shuttle.rs/)

## Getting Started

### Clone the Repository

```sh
git clone https://github.com/yourusername/todo-app.git
cd todo-app
```

### Run the Application

Use Shuttle to run the application:

```sh
cargo shuttle run
```

The application will start on `http://localhost:8000`.

## Project Structure

```
my-todo-app
├── src
│   ├── main.rs
│   ├── models.rs
├── templates
│   └── index.html
├── Cargo.toml
```

- `src/main.rs`: Contains the main application code and routes.
- `src/models.rs`: Contains the data models for the application.
- `templates/index.html`: The front-end HTML file.
- `Cargo.toml`: Rust project configuration file.

## Front-End

The front-end is a single HTML file located at `templates/index.html`, using:

- [Pico.css](https://picocss.com/) for minimal CSS styling.
- [Alpine.js](https://alpinejs.dev/) for reactive JavaScript.

### External Resources

Pico.css and Alpine.js are included via CDN:

```html
<link rel="stylesheet" href="https://unpkg.com/@picocss/pico@1.4.0/css/pico.min.css">
<script src="https://unpkg.com/alpinejs" defer></script>
```

## API Endpoints

- `GET /todos`: Retrieve all TODO items.
- `GET /todos/:id`: Retrieve a specific TODO item by ID.
- `POST /todos`: Add a new TODO item.
- `DELETE /todos/:id`: Delete a TODO item by ID.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [Shuttle.rs](https://www.shuttle.rs/)
- [Pico.css](https://picocss.com/)
- [Alpine.js](https://alpinejs.dev/)
