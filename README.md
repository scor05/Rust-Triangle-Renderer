# Renderer de Triángulos

## Descripción y Estructura del Código

En este repositorio se encuentra mi código realizado para el renderer de modelos .obj en Rust, el cual toma un archivo .obj y lo renderiza en pantalla utilizando los triángulos del objeto. Actualmente el código está configuardo únicamente para renderizar los triángulos para el laboratorio #5, por lo que el .obj anterior ya no se utiliza.

El código está estructurado de la siguiente forma:

```bash
├── assets
│   └── obj
│       └── nave.obj
├── Cargo.lock
├── Cargo.toml
├── flamegraph.svg
├── perf.data
├── README.md
├── screenshots
│   └── triangles
│       ├── horizontal_and_vertical_edges__filled_barycentric.png
│       ├── horizontal_and_vertical_edges__filled_scanline.png
│       ├── horizontal_and_vertical_edges__wireframe.png
│       ├── original_and_reversed_winding__filled_barycentric.png
│       ├── original_and_reversed_winding__filled_scanline.png
│       ├── original_and_reversed_winding__wireframe.png
│       ├── partially_outside_and_collinear__filled_barycentric.png
│       ├── partially_outside_and_collinear__filled_scanline.png
│       ├── partially_outside_and_collinear__wireframe.png
│       ├── small_and_large__filled_barycentric.png
│       ├── small_and_large__filled_scanline.png
│       └── small_and_large__wireframe.png
└── src
    ├── algorithms
    │   ├── barycentric.rs
    │   ├── mod.rs
    │   ├── scanline.rs
    │   ├── triangle.rs
    │   └── wireframe.rs
    ├── framebuffer.rs
    ├── line.rs
    ├── main.rs
    └── model.rs
```

En `./assets/obj/nave.obj` está el objeto que yo utilicé para tomar la captura en la sección de "Cómo ejecutar" para demostrar el funcionamiento del código, el cual es el modelo realizado para el lab #3 de la clase de gráficas (la nave espacial). En `./src/` se encuentran todos los archivos del código separados por su función: el framebuffer, línea (dibuja líneas entre dos vectores), model (para la data del .obj) y el main.

## Cómo ejecutar

El proyecto está diseñado para que se pueda ejecutar el código desde la carpeta raíz del repositorio con

```bash
cargo run --release
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
