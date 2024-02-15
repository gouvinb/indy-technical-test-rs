# Indy technical test in Rust

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

Nothing for now.
