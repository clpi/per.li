FROM rustlang/rust:nighly

WORKDIR /usr/src/papi

COPY . .

RUN cargo install cargo-make

RUN cargo make install .

CMD ["/usr/src/papi/papi", "serve"]
