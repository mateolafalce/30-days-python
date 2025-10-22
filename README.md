<div align="center">

# 🐍 30 Days Of Python

</div>

30 days of Python programming challenge is a step-by-step guide to learn the Python programming language in 30 days!

## 📋 Prerequisites

- **For compilation:** [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- **For Docker:** [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)

## 🚀 Installation & Deployment

### Method 1: Compile and Run Locally

1. **Clone this repository:**

```bash
git clone https://github.com/mateolafalce/30-days-python.git
cd 30-days-python
```

2. **Compile & Run:**

```bash
cargo run --release
```

3. **Access the application:**

```
http://localhost:5001
```

### Method 2: Using Docker Compose

1. **Clone this repository:**

```bash
git clone https://github.com/mateolafalce/30-days-python.git
cd 30-days-python
```

2. **Start the service:**

```bash
docker-compose up -d
```

3. **Access the application:**

```
http://localhost:5001
```

4. **Stop the service:**

```bash
docker-compose down
```

## 📝 Additional Commands

### Development Mode

For faster compilation during development:

```bash
cargo run
```

### View Docker Logs

```bash
docker-compose logs -f
```

### Rebuild Docker Image

```bash
docker-compose up --build -d
```

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- [30 Days of Python](https://github.com/Asabeneh/30-Days-Of-Python) for providing the data
- [Axum](https://github.com/tokio-rs/axum) for the excellent web framework
