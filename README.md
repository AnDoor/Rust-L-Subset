# 📄 Documentacion

L es un lenguaje de programación "mini" inspirado en la sintaxis de Rust, pero enfocado únicamente en cálculos matemáticos.
Identificadores: Solo reconoce números enteros de 32 bits (i32).
Palabras clave: Utiliza let para indicar el inicio de una expresión (siguiendo el estilo de Rust).
Símbolos: Reconoce los operadores básicos (+, -, *, /) y paréntesis () para dar prioridad a ciertas operaciones.

# 📁 Estrcutura del repositorio
```text
proyecto_L/
├── Cargo.toml  # Dependencias (lalrpop, lalrpop-util)
├── build.rs  # Configuración para compilar la gramática
├── src/
│ ├── main.rs  # Punto de entrada
│ └── gramatica.lalrpop # Definición de nuestro Lexer/Parser L
└── README.md  # Documentación
```
## 💾 Instalacion

La instalacion y la compilacion esta basada en Windows, por lo tanto, en Linux no funciona. <br></br>
Asegúrate de tener Rust instalado ejecutando `rustc --version` en tu terminal

1. Clona el repositorio en una carpeta. <br></br>
   `https://github.com/AnDoor/Rust-L-Subset.git`
   `cd Rust-L-Subset`
   
3.  Instalar por powershell Windows Rustup para ejecutar: <br></br>
`rustup default stable-x86_64-pc-windows-gnu`

Esto descargará la versión de Rust diseñada para trabajar con **GCC/MinGW**.
· La establecerá como predeterminada.
· Dejará de buscar `link.exe` de Microsoft y empezará a buscar `gcc` o `ld`, que ya tienes instalados en tus carpetas de MinGW.

3. Compilar el codigo <br></br>
`Cargo build`<br></br>
`Cargo Run "2 + 3"`
