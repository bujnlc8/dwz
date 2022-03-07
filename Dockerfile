FROM ubuntu:18.04
WORKDIR dwz
COPY target/release/dwz .
RUN apt update && apt install libmysqlclient-dev -y && rm -rf /var/lib/apt/lists/*
COPY templates/ templates/
CMD ["./dwz"]
