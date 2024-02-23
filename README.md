# Technical test in Rust

## Roadmap

:warning: Warning:

- Order is irrelevant
- I had a few interruptions while doing the exercise (personal reason)

| Steps                                  | Temps  | Note                                    |
|----------------------------------------|--------|-----------------------------------------|
| Project initialization                 | ~10min |                                         |
| Brainstorming                          | ~60min |                                         |
| Data model & validation part           | ~45min |                                         |
| Brainstorming                          | ~10min |                                         |
| Handmade database                      | ~20min |                                         |
| `get_promocode_list` endpoint          | ~5min  |                                         |
| `put_promocode` endpoint               | ~5min  |                                         |
| `delete_promocode` endpoint            | ~5min  |                                         |
| _Indy information request_             | ~90min |                                         |
| Brainstorming                          | ~90min | **No answer to my questions from Indy** |
| `get_promocode` endpoint without meteo | ~60min | _Can be simplified_                     |
| `get_promocode` endpoint with meteo    | ~30min | _Can be simplified_                     |
| 1st review                             | ~30min |                                         |
| Limit break restrictions               | ~5min  |                                         |
| 2nd review                             | ~30min |                                         |
| Github action                          | ~10min | _`act` dl 75Gb on my pc_                |
| `actix_web` to `ntex`                  | ~20min | _With testing_                          |

**Total**: 8h and 45 min

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

- [serde-rs/serde#939-939514114](https://github.com/serde-rs/serde/issues/939#issuecomment-939514114)

## What's next?

- Fix last warnings/todos
- Make a client module
- Add logger
- Make builder pattern for some struct (with validation ect...)
- Use a embedded db
- Generate OpenAPI specs
- Generate API docs for Github Pages (Although the code is on a public platform,
  it remains private, so no docs.rs)

