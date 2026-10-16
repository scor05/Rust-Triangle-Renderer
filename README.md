# Renderer de Triángulos

## Descripción y Estructura del Código

En este repositorio se encuentra mi código realizado para el renderer de modelos .obj en Rust, el cual toma un archivo .obj y lo renderiza en pantalla utilizando los triángulos del objeto.

El código está estructurado de la siguiente forma:

```bash
├── assets
│   └── obj
│       └── nave.obj
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── framebuffer.rs
    ├── line.rs
    ├── main.rs
    └── model.rs
```

En `./assets/obj/nave.obj` está el objeto que yo utilicé para tomar la captura en la sección de "Cómo ejecutar" para demostrar el funcionamiento del código, el cual es el modelo realizado para el lab #3 de la clase de gráficas (la nave espacial). En `./src/` se encuentran todos los archivos del código separados por su función: el framebuffer, línea (dibuja líneas entre dos vectores), model (para la data del .obj) y el main.

## Cómo ejecutar

El proyecto está diseñado para que se pueda ejecutar el código desde la carpeta raíz del repositorio con

```bash
cargo run
```

Y al ejecutar eso se debería de ver una ventana con el modelo .obj similar a esta:
<img width="1600" height="863" alt="image" src="https://github.com/user-attachments/assets/2b9acddb-43c8-46b6-ab8f-8794149b9be0" />

## Controles

Para visualizar el modelo y rotarlo/escalarlo, implementé los siguientes controles:

- W/S: rotación en y
- A/D: rotación en x
- Z/X: rotación en z
- Q/E: escalar y achiquitar modelo
- Flechas del teclado: trasladar el modelo
