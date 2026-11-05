// Forja WASM - Playground interactivo en el navegador
// Expone el compilador de Forja como funciones WASM
// con ejemplos, URL sharing y más

use forja::error::limpiar_ansi;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

/// Caché en memoria de bytecode compilado, indexado por hash del source.
/// En WASM no hay filesystem, así que la caché incremental de disco no
/// aplica; esta caché evita recompilar el mismo código en ejecuciones
/// sucesivas del playground (que es el caso de uso dominante).
static CACHE_BYTECODE: Mutex<Option<HashMap<u64, (Vec<forja::bytecode::Opcode>, Vec<forja::bytecode::ContratoBytecode>)>>> =
    Mutex::new(None);

fn hash_source(source: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

/// Transpila código Forja a Rust (transpilación; la API WASM conserva el
/// nombre `compilar` por compatibilidad con el JS existente).
#[wasm_bindgen]
pub fn compilar(codigo: &str) -> String {
    match forja::transpilar_a_rust(codigo) {
        Ok(rust_code) => rust_code,
        Err(errors) => {
            let msgs: Vec<String> = errors
                .iter()
                .map(|e| limpiar_ansi(&e.to_string()))
                .collect();
            format!("Error: {}", msgs.join("\n"))
        }
    }
}

/// Compila y ejecuta código Forja en la VM, devolviendo el output.
/// El navegador no interpreta ANSI: los errores se devuelven sin color.
#[wasm_bindgen]
pub fn forja_ejecutar(source: &str) -> Result<String, JsError> {
    use forja::vm_fast::ForjaFast;

    // 1. Intentar cache hit
    let source_hash = hash_source(source);
    let cached = {
        let guard = CACHE_BYTECODE.lock().unwrap();
        if let Some(map) = guard.as_ref() {
            map.get(&source_hash).cloned()
        } else {
            None
        }
    };

    let (bytecode, contratos) = if let Some((bc, ct)) = cached {
        (bc, ct)
    } else {
        // Cache miss: compilar normalmente (resuelve imports embebidos)
        let (bc, ct) = forja::compilar_pipeline_completa(source)
            .map_err(|e| JsError::new(&limpiar_ansi(&e)))?;
        // Guardar en caché
        {
            let mut guard = CACHE_BYTECODE.lock().unwrap();
            let map = guard.get_or_insert_with(HashMap::new);
            // Limitar tamaño del caché a 64 entradas
            if map.len() >= 64 {
                map.clear();
            }
            map.insert(source_hash, (bc.clone(), ct.clone()));
        }
        (bc, ct)
    };

    let mut vm = ForjaFast::new();
    #[cfg(target_pointer_width = "64")]
    vm.set_max_inst(10_000_000_000);
    #[cfg(not(target_pointer_width = "64"))]
    vm.set_max_inst(2_000_000_000);
    vm.contratos = contratos;
    vm.verificar_contratos = true;
    vm.cargar_bytecode(bytecode);
    vm.ejecutar()
        .map_err(|e| JsError::new(&limpiar_ansi(&format!("{}", e))))?;
    Ok(vm.obtener_output().to_vec().join("\n"))
}
