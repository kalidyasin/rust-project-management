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
