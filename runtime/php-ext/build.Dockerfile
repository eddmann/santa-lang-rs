FROM rust:1.70.0-bullseye

RUN apt update \
 && apt -y install lsb-release apt-transport-https ca-certificates \
 && wget -O /etc/apt/trusted.gpg.d/php.gpg https://packages.sury.org/php/apt.gpg \
 && echo "deb https://packages.sury.org/php/ $(lsb_release -sc) main" | tee /etc/apt/sources.list.d/php.list \
 && apt update \
 && apt install php8.2-cli php8.2-dev llvm-dev libclang-dev clang -y
