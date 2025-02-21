Загрузить на Rpi:

cargo build --target="aarch64-unknown-linux-gnu" --release; scp target/aarch64-unknown-linux-gnu/release/rpi user@target:/home/user/

Запустить на локальной машине:

cargo run
