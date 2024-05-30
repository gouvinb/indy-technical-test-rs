# Technical test in Rust

## Project layout

```text
.
├── etc
│  └── http          // `http` files for testing request (Jetbrains product pack
│                    // only)
├── promocode-client // Client sources (bonus)
├── promocode-models // Shared model for client and server
│  └── src
│     ├── data       // Low level models
│     ├── request    // Request models
│     └── response   // Response models
├── promocode-server // Server sources
├── promocode-util   // Shared code
├── Cargo.toml
├── LICENSE.md
└── README.md
```

## References

- [chrono](https://github.com/chronotope/chrono)
- [clap](https://github.com/clap-rs/clap)
- [env_logger](https://github.com/rust-cli/env_logger)
- [log](https://github.com/rust-lang/log)
- [ntex](https://github.com/ntex-rs/ntex)
- [openweather_sdk](https://github.com/jt-rose/openweather_sdk)
- [serde](https://github.com/serde-rs/serde)
- [serde_json](https://github.com/serde-rs/json)

- [serde-rs/serde#939-939514114](https://github.com/serde-rs/serde/issues/939#issuecomment-93951414)
