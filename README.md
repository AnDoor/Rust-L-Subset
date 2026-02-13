# 📄 Documentacion


# Estrcutura del repositorio
proyecto_L/
├── Cargo.toml  # Dependencias (lalrpop, lalrpop-util)
├── build.rs  # Configuración para compilar la gramática
├── src/
│ ├── main.rs  # Punto de entrada
│  └── gramatica.lalrpop # Definición de nuestro Lexer/Parser L
└── README.md  # Documentación

## Instalacion

La instalacion y la compilacion esta basada en Windows, por lo tanto, en Linux no funciona.

1.  Instalar por powershell Windows Rustup para ejecutar: 
`rustup default stable-x86_64-pc-windows-gnu`

Esto descargará la versión de Rust diseñada para trabajar con **GCC/MinGW**.
· La establecerá como predeterminada.
· Dejará de buscar `link.exe` de Microsoft y empezará a buscar `gcc` o `ld`, que ya tienes instalados en tus carpetas de MinGW.

2. Compilar el codigo 
`Cargo build`
`Cargo Run "2 + 3"`
