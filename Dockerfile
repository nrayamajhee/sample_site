FROM rust:latest as builder

COPY . .
RUN cargo build --release
RUN mv target/release/sample_site /opt/sample_site
EXPOSE 8080
CMD /opt/sample_site
