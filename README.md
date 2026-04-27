# 🕹️ CHIP-8 BetilloEmulator

Un emulador funcional de la arquitectura CHIP-8 desarrollado en **Rust**, capaz de ejecutar ROMs clásicas y programas personalizados.

![Logo Betillo en el Emulador](images/BETILLO_LOGO.png)

### Características
* **Ciclo de CPU completo:** Implementación de *Fetch, Decode y Execute*.
* **Gráficos:** Renderizado mediante la librería `minifb` con escalado de píxeles.
* **Memoria:** Gestión de 4096 bytes de RAM y stack de 16 niveles.
* **Fuentes personalizadas:** Carga de un set de fuentes hexadecimales en la memoria base.

### Arquitectura del Proyecto
El emulador está dividido en módulos para mantener una separación de responsabilidades clara:

1.  **`chip8.rs`**: Define el "Estado" de la máquina (Registros, Memoria, Pantalla).
2.  **`cpu.rs`**: Contiene el motor lógico (Opcodes y ciclo de instrucciones).
3.  **`main.rs`**: Maneja el loop de renderizado, la carga de la ROM y la interacción con el usuario.

---

## 🧠 Desafíos Técnicos y Aprendizaje

### 1. ¿Cómo manejé el "Endiness" y la unión de bytes?

- CHIP-8 usa *Big-endian*. Tuve que combinar dos bytes de la memoria para formar una sola instrucción (opcode) de 16 bits, desplazando el primero a la izquierda.

### 2. ¿Cuál fue el reto del Opcode de dibujo (0xDXYN)?

- Este es el opcode más complejo. Implementé la lógica de **XOR** para los píxeles (que es como CHIP-8 detecta colisiones) y el uso del registro `VF` como bandera de colisión si un píxel se apaga. Además, manejé el "wrapping" para que el dibujo no rompa la memoria si sale de las coordenadas.

### 3. ¿Cómo controlé la velocidad de ejecución?

- Si la CPU corre a la velocidad de mi procesador moderno, el juego termina en un milisegundo. Implementé un límite de 60Hz para los timers y ejecuté 8 instrucciones por frame para balancear la velocidad del juego.

---

## 🎨 Mi Propia ROM: "BETILLO.ch8"
Como prueba de concepto final, no solo corrí juegos existentes, sino que **diseñé mi propio logo en lenguaje de máquina**.
* **Proceso:** Dibujé los bytes en binario para formar las letras, los cargué en una ROM y utilicé el registro `I` y el opcode de dibujo para proyectarlo en pantalla.

---

## 🛠️ Instalación y Uso

### Opción A: Descargar el Binario (Recomendado)
Si solo quieres probarlo, ve a la sección de [Releases](https://github.com/betilloXann/chip-8/releases) y descarga el archivo `.zip` para tu sistema operativo.

1. Descomprime el archivo.
2. Asegúrate de tener la ROM en la ruta `src/roms/BETILLO.ch8`.
3. Ejecuta el binario:
   ```bash
   ./chip-8

### Opción B: Compilar desde el Código Fuente
Requiere tener instalado Rust/Cargo

1. Clona el repositorio:

```bash
git clone https://github.com/betilloXann/chip-8.git
cd chip-8
```

2. Compila en modo optimizado (Release):
`cargo build --release`

3. Ejecuta el emulador:
`./target/release/chip-8`

Desarrollado con 🦀 por betilloXann
