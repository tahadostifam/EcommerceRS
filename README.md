# Ecommerce RESTful API in Rust

Welcome to the **Ecommerce RESTful API** project! This is an open-source, scalable, and modular ecommerce backend built with **Rust**. The goal of this project is to provide a high-performance, secure, and easy-to-extend foundation for building ecommerce applications. Whether you're a developer looking to contribute or someone building an ecommerce platform, this project is designed to grow and improve over time.

---

## Features

- [x] **RESTful API**: Fully compliant with REST principles for easy integration with frontend applications.
- [x] **Modular Architecture**: Follows Hexagonal Architecture (Ports and Adapters) for clean separation of concerns.
- [x] **Authentication & Authorization**: Secure Refresh Token Authentication and role-based access control.
- [x] **Product Management**: Create, read, update, and delete products with categories and inventory tracking.
- [x] **User Management**: User registration, login, and profile management.
- [x] **Search & Filtering**: Advanced search and filtering for products.
- [x] **Docker Support**: Easy deployment with Docker.
- [ ] **Order Management**: Handle orders, payments, and shipping status.
- [ ] **Pagination**: Efficient handling of large datasets with pagination.

---

## Tech Stack

- **Language**: Rust
- **Web Framework**: Actix Web
- **Database**: PostgreSQL (with `diesel` as ORM) + Redis
- **Authentication**: RefreshToken & Google OAuth2
- **Serialization**: Serde
- **Validation**: Validator crate
- **Testing**: Cargo test, integration tests
- **Containerization**: Docker

---

## Getting Started

### Installation

1. **Clone the repository**:

```bash
   git clone https://github.com/tahadostifam/EcommerceRS.git
   cd EcommerceRS
```

2. ** Setup Development Environment

```bash
docker-compose -f ./docker-compose.devel.yml up -d postgres redis
make migrate-dev
```

3. ** Bootstrap RESTAPI

```bash
cargo run -p restapi
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

If you have any questions or suggestions, feel free to reach out:

- **Email:** <mr.tahadostifam@gmail.com>
- **Github Issues** [Create Issue](https://github.com/tahadostifam/EcommerceRS/issues)
