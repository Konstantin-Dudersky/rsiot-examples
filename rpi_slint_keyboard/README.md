cargo install cross

cross build --target aarch64-unknown-linux-gnu --release; scp target/aarch64-unknown-linux-gnu/release/rpi_slint_keyboard user@target:/home/user/

## Запуск на целевом устройстве

```bash
SLINT_KMS_ROTATION=270 /home/user/rpi_slint_keyboard
```
