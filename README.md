# RUST self signed HTTP/Server

This project will show how to create http server/client example with Rust programing language.

## HTTP server

HTTP server is using actix-web library

- Generating self signed certificate

```
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```

## HTTP client

HTTP client is using reqwest library.
