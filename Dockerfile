FROM rust:latest

WORKDIR /app

# 前提インストール
RUN dpkg --add-architecture armhf
RUN apt update && apt upgrade -y
RUN apt install -y libpq-dev

# ソースコードのコピー
COPY . .

## ビルド
#RUN cargo build --release

# release
#CMD [ "/app/target/debug/sakura-api" ]

# develop
CMD ["cargo", "run"]