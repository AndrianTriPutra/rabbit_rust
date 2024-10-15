# rabbit_rust

## getting started
- if needed, you can change configuration at dir config
- publisher : cargo run publisher ./config/.config.yaml trace
- consumer  : cargo run consumer ./config/.config.yaml trace

## docker
- rabbit : docker compose -f docker-compose.yml -p rabbitmq up --remove-orphans -d --build
