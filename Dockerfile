FROM rust:latest as builder

COPY . .
RUN cargo build --release
RUN mv target/release/sample_site /opt/sample_site
CMD /opt/sample_site
