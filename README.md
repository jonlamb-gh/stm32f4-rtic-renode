# air-gradient-pro-rs

TODO
* update my fork of stm32-eth
  - see https://github.com/stm32-rs/stm32-eth/pulls (filter modes PR is WIP)
* try out renode emulation for tests
  - can also emulate the peripherals in Rust: https://antmicro.com/blog/2021/07/rust-peripheral-support-in-renode/
  - https://github.com/antmicro/renode-rust-example


PHY's in here, and touchscreen too
https://github.com/renode/renode/blob/master/platforms/boards/stm32f7_discovery-bb.repl

```
renode renode/emulate.resc

gdb-multiarch target/thumbv7em-none-eabihf/debug/air-gradient-pro-rs
```
