// Forja WASM - Playground interactivo en el navegador
// Expone el compilador de Forja como funciones WASM
// con ejemplos, URL sharing y más

use wasm_bindgen::prelude::*;

/// Transpila código Forja a Rust (transpilación; la API WASM conserva el
/// nombre `compilar` por compatibilidad con el JS existente).
#[wasm_bindgen]
pub fn compilar(codigo: &str) -> String {
    match forja::transpilar_a_rust(codigo) {
        Ok(rust_code) => rust_code,
        Err(errors) => {
            let msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            format!("Error: {}", msgs.join("\n"))
        }
    }
}

/// Compila y ejecuta código Forja en la VM, devolviendo el output
#[wasm_bindgen]
pub fn forja_ejecutar(source: &str) -> Result<String, JsError> {
    let output = forja::ejecutar(source).map_err(|e| JsError::new(&e))?;
    Ok(output.join("\n"))
}

/// Obtener ejemplos disponibles como JSON
#[wasm_bindgen]
pub fn obtener_ejemplos() -> String {
    let ejemplos = vec![
        (
            "Hola Mundo",
            r#"importar std/io
escribir("¡Hola, Mundo!")"#,
        ),
        (
            "Variables",
            r#"importar std/io
variable nombre = "Ana"
variable edad = 25
escribir("Me llamo " + nombre + " y tengo " + edad + " años")"#,
        ),
        (
            "Funciones",
            r#"importar std/io
funcion fibonacci(n) {
    si (n <= 1) { retornar n }
    retornar fibonacci(n - 1) + fibonacci(n - 2)
}
para (variable i = 0; i < 10; i = i + 1) {
    escribir(fibonacci(i))
}"#,
        ),
        (
            "Clases",
            r#"importar std/io
clase Persona {
    nombre
    constructor(nombre) {
        este.nombre = nombre
    }
    funcion saludar() {
        escribir("Hola, soy " + este.nombre)
    }
}
variable p = nuevo Persona("Ana")
p.saludar()"#,
        ),
        (
            "String Interpolation",
            r#"importar std/io
variable nombre = "Carlos"
variable edad = 30
escribir("Hola ${nombre}, tenés ${edad} años")"#,
        ),
        (
            "Resultado/Option",
            r#"importar std/io
funcion dividir(a: Entero, b: Entero) -> Resultado<Entero, Texto> {
    si (b == 0) { retornar Error("No se puede dividir por cero") }
    retornar Ok(a / b)
}
funcion ejecutar() -> Resultado<Entero, Texto> {
    variable resultado = dividir(10, 2)?
    escribir(resultado)
    retornar Ok(resultado)
}
ejecutar()"#,
        ),
        (
            "Rasgos",
            r#"importar std/io
rasgo Saludador {
    funcion saludar()
}
clase Persona {
    nombre
    constructor(n) { este.nombre = n }
}
implementa Saludador para Persona {
    funcion saludar() {
        escribir("Hola, soy " + este.nombre)
    }
}
variable p = nuevo Persona("Ana")
p.saludar()"#,
        ),
        (
            "Genéricos",
            r#"importar std/io
funcion identidad<T>(valor: T) -> T {
    retornar valor
}
escribir(identidad(42))
escribir(identidad("hola"))"#,
        ),
        (
            "Concurrencia",
            r#"importar std/io
variable tx, rx = canal()
variable h = hilo {
    tx.enviar(42)
}
variable dato = rx.recibir()
escribir(dato)
h.unir()"#,
        ),
        (
            "Input de usuario",
            r#"importar std/io
variable nombre = leer("¿Cómo te llamas? ")
escribir("Hola, " + nombre + "!")"#,
        ),
        (
            "Fibonacci recursivo",
            r#"importar std/io
funcion fib(n) {
    si (n < 2) { retornar n }
    retornar fib(n-1) + fib(n-2)
}
variable n = 10
escribir("fib(" + n + ") = " + fib(n))"#,
        ),
        (
            "Números primos",
            r#"importar std/io
funcion es_primo(n) {
    si (n < 2) { retornar falso }
    variable i = 2
    mientras (i * i <= n) {
        si (n % i == 0) { retornar falso }
        i = i + 1
    }
    retornar verdadero
}
para (variable i = 1; i <= 20; i = i + 1) {
    si (es_primo(i)) {
        escribir(i + " es primo")
    }
}"#,
        ),
        (
            "Arreglos",
            r#"importar std/io
variable numeros = [3, 1, 4, 1, 5, 9, 2, 6]
variable suma = 0
para (variable i = 0; i < longitud(numeros); i = i + 1) {
    suma = suma + numeros[i]
}
escribir("Suma: " + suma)
escribir("Máximo: " + maximo(numeros))"#,
        ),
        (
            "Mapas",
            r#"importar std/io
variable edades = {"Ana": 25, "Luis": 30, "Eva": 28}
variable nombres = ["Ana", "Luis", "Eva"]
para (variable i = 0; i < longitud(nombres); i = i + 1) {
    variable nombre = nombres[i]
    escribir(nombre + " tiene " + edades[nombre] + " años")
}"#,
        ),
        (
            "Enums y match",
            r#"importar std/io
tipo Forma = Circulo(Decimal) | Rectangulo(Decimal, Decimal)
funcion area(f: Forma) -> Decimal {
    coincidir (f) {
        caso Circulo(r) -> { retornar 3.14159 * r * r }
        caso Rectangulo(a, b) -> { retornar a * b }
    }
}
escribir(area(Circulo(2.0)))
escribir(area(Rectangulo(3.0, 4.0)))"#,
        ),
        (
            "Ordenamiento burbuja",
            r#"importar std/io
variable arr = [5, 2, 9, 1, 7, 3]
variable n = longitud(arr)
para (variable i = 0; i < n; i = i + 1) {
    para (variable j = 0; j < n - i - 1; j = j + 1) {
        si (arr[j] > arr[j + 1]) {
            variable tmp = arr[j]
            arr[j] = arr[j + 1]
            arr[j + 1] = tmp
        }
    }
}
escribir(arr)"#,
        ),
        (
            "GUI: contador",
            r#"importar std/io
variable contador = 0
funcion incrementar() {
    contador = contador + 1
}
funcion main() {
    columna {
        titulo("Contador")
        escribir("Valor: " + contador)
        boton("Incrementar", incrementar)
    }
}"#,
        ),
        (
            "JSON",
            r#"importar std/io
importar std/json
funcion ejecutar() -> Resultado<Texto, ErrorJSON> {
    variable datos = {"nombre": "Ana", "edad": 25, "activo": verdadero}
    variable texto = serializar(datos)?
    escribir(texto)
    variable recuperado = parsear(texto)?
    escribir(recuperado["nombre"])
    retornar Ok(texto)
}
ejecutar()"#,
        ),
    ];
    serde_json::to_string(&ejemplos).unwrap()
}

/// Decodificar código desde URL (percent-encoding)
#[wasm_bindgen]
pub fn decodificar_url(codigo_encoded: &str) -> String {
    urlencoding::decode(codigo_encoded)
        .map(|s| s.to_string())
        .unwrap_or_else(|_| String::new())
}

/// Codificar código para compartir via URL (percent-encoding)
#[wasm_bindgen]
pub fn codificar_url(codigo: &str) -> String {
    urlencoding::encode(codigo).to_string()
}

/// Obtener versión de Forja
#[wasm_bindgen]
pub fn version() -> String {
    let pkg_version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0");
    pkg_version.to_string()
}

/// Tokeniza código Forja y devuelve los tokens como JSON
#[wasm_bindgen]
pub fn forja_tokenizar(source: &str) -> Result<String, JsError> {
    let mut lexer = forja::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|errors| {
        let msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        JsError::new(&msgs.join("\n"))
    })?;

    let json = tokens
        .iter()
        .map(|t| {
            format!(
                r#"{{"kind":"{}","linea":{},"columna":{}}}"#,
                t.kind, t.linea, t.columna
            )
        })
        .collect::<Vec<_>>()
        .join(",\n  ");

    Ok(format!("[\n  {}\n]", json))
}
