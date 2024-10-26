# rabbit_rust

## getting started
- if needed, you can change configuration at dir config
- publisher : cargo run publisher ./config/.config.yaml trace
- consumer  : cargo run consumer ./config/.config.yaml trace

## docker
- rabbit : docker compose -f docker-compose.yml -p rabbitmq up --remove-orphans -d --build

## documentation
- [medium](https://andriantriputra.medium.com/be-rust-how-to-implement-rabbit-in-rust-239ceed71895)

- [youtube](https://youtu.be/CWvA6emz8iY)