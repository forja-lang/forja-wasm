# forja-wasm

Compilador de **Forja (fa)** compilado a **WebAssembly** para el [Playground interactivo](https://forja-lang.github.io/docs/playground).

## ¿Qué hace?

Expone el núcleo del compilador de Forja (léxico, parseo, ejecución en VM) como funciones WASM llamables desde JavaScript. Permite ejecutar código Forja directamente en el navegador sin instalar nada.

## Funciones exportadas

| Función JS | Descripción |
|---|---|
| `compilar(codigo)` | Transpila código Forja a Rust |
| `forja_ejecutar(source)` | Compila y ejecuta en la VM, devuelve el output |
| `forja_tokenizar(source)` | Tokeniza el código y devuelve los tokens como JSON |
| `codificar_url(codigo)` | Codifica código para compartir via URL |
| `decodificar_url(codigo_encoded)` | Decodifica código desde URL |
| `obtener_ejemplos()` | Devuelve ejemplos precargados como JSON |
| `version()` | Versión del compilador |

## Build

```bash
# Requiere wasm-pack o wasm-bindgen
cargo build --package forja-wasm --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/forja_wasm.wasm --out-dir wasm-output --target web
wasm-opt --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext -O3 wasm-output/forja_wasm_bg.wasm -o wasm-output/forja_wasm_bg.wasm
```

## Dependencias

- `wasm-bindgen = "0.2"` — Bindings JS↔WASM
- `forja = { path = "../.." }` — Núcleo del compilador
- `serde / serde_json` — Serialización de ejemplos y tokens
- `urlencoding` — Compartir código via URL
