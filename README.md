# esp-gpio-expander :crab:
## Description 
Example of work with `mcp23017` (aka. `cjmcu-2317`) GPIO expander with Espressif chips.
For this example I took a classic pushbutton and tried to connect it with Espressif board via GPIO expander on different Espressif boards: 
 - [ESP32](https://www.espressif.com/en/products/socs/esp32)
 - [ESP32-S2](https://www.espressif.com/en/products/socs/esp32-s2)
 - [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3)
<img src = "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse2.mm.bing.net%2Fth%3Fid%3DOIP.TaLNEE-MfCua9wQVEZDWvAHaFj%26pid%3DApi&f=1&ipt=a646f1546491c8f3761747effe4f315677187bebb92f23a1ffb2d9aa7e5e439b&ipo=images" width = "50%" heigth="40%">
<br>
> cjmcu-2317 GPIO expander

## Build and flash

**Warning**
Choose the corresponding options for your chip in `Cargo.toml`

### Build for ESP32-S3-USB-OTG

```
cargo espflash --release --monitor --target xtensa-esp32s3-none-elf 
```

### Build for ESP32

```
cargo espflash --release --monitor --target xtensa-esp32-none-elf 
```

### Build for ESP32-S2

```
cargo espflash --release --monitor --target xtensa-esp32s2-none-elf 
```

### Build for ESP32-C3

```
cargo espflash --release --monitor --target riscv32imac-unknown-none-elf
```

