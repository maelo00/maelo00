# Plugin para Rust

Este repositorio contiene un ejemplo mínimo de un **plugin en Rust** compilado como biblioteca dinámica (`cdylib`).

## Compilar

```bash
cargo build --release
```

El artefacto generado estará en:

- Linux: `target/release/librust_plugin.so`
- macOS: `target/release/librust_plugin.dylib`
- Windows: `target/release/rust_plugin.dll`

## API expuesta (C ABI)

```c
const char* plugin_version(void);
const char* plugin_greet(const char* name);
```

- `plugin_version` devuelve una cadena estática con la versión.
- `plugin_greet` devuelve una cadena estática con un saludo simple.

> Nota: las cadenas devueltas son estáticas y no deben liberarse.
