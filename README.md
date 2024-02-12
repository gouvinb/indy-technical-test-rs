# Indy technical test in Rust

## Roadmap

:warning: Warning:

- Order is irrelevant
- I had a few interruptions while doing the exercise (personal reason)

| Steps                        | Temps  | Note                                    |
|------------------------------|--------|-----------------------------------------|
| Project initialization       | ~10min |                                         |
| Brainstorming                | ~60min |                                         |
| Data model & validation part | ~45min |                                         |
| Brainstorming                | ~10min |                                         |
| Handmade database            | ~20min |                                         |
| get_promocode_list endpoint  | ~5min  |                                         |
| put_promocode endpoint       | ~5min  |                                         |
| delete_promocode endpoint    | ~5min  |                                         |
| _Indy information request_   | ~90min |                                         |
| Brainstorming                | ~90min | **No answer to my questions from Indy** |
| get_promocode endpoint       | ~60min | _Can be simplified_                     |
| ...                          | ~XXmin |                                         |

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
