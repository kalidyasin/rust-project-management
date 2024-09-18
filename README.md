# Rust Project Management System

This is a Rust-based Project Management System API built using the Actix-web framework. It provides endpoints for user registration and login.

## Features

* User Registration
* User Login

## Endpoints

### GET /

Returns a welcome message for the Project Management API.

### POST /login

Authenticates a user.

* Request Body:
	+ `email`: The user's email address.
	+ `password`: The user's password.
* Response:
	+ `token`: A JSON Web Token (JWT) that can be used to authenticate the user.

### POST /signup

Registers a new user.

* Request Body:
	+ `name`: The user's name.
	+ `email`: The user's email address.
	+ `password`: The user's password.
* Response:
	+ `message`: A success message indicating that the user has been registered.

## Project Structure

The project is structured as follows:

* [src/main.rs](cci:7://src/main.rs:0:0-0:0): The main entry point of the application.
* `Cargo.toml`: The project's configuration file.
* `Dockerfile`: The Dockerfile used to build the project's Docker image.

## Getting Started

### Cloning the Repository

To clone the repository, run the following command:
```bash
git clone https://github.com/your-username/project-management.git
```
Replace `your-username` with your actual GitHub username.

### Forking the Repository
To fork the repository, follow these steps:

1. Log in to your GitHub account.
2. Navigate to the repository's page.
3. Click the "Fork" button in the top-right corner of the page.
4. Choose the account where you want to fork the repository.
5. Click "Create fork" to create a new fork of the repository.

### Building the Image
To build the Docker image, run the following command:
```bash
docker build -t project-management .
```
This will build the Docker image using the instructions in the `Dockerfile`.

### Running the Container
To run the Docker container, run the following command:
```bash
docker run -p 8080:8080 project-management
```
This will start a new container from the `project-management` image and map port 8080 on the host machine to port 8080 in the container.

## Dockerfile

The Dockerfile is set up to create a multi-stage build for the Rust project. It uses `rust:alpine` for the build stage and `alpine:latest` for the run stage.

```dockerfile
# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:alpine AS builder
WORKDIR /code

RUN apk add --no-cache gcc g++ musl-dev

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init

COPY Cargo.toml Cargo.toml

RUN cargo fetch

# copy app files
COPY src src

# compile app
RUN cargo build --release

###############
## run stage ##
###############
# FROM debian:buster-slim
FROM alpine:latest
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/project-management project-management

# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/project-management" ]
```

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/kalidyasin/rust-project-management/blob/main/LICENSE) file for more details.
