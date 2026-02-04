# 🃏 MPoker - Desktop Poker Game

<div align="center">
  <img src="https://github.com/MoiBaldenegro/mpoker/blob/main/public/tauri.svg" alt="Tauri Logo" width="100" />
  <img src="https://rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust Logo" width="150" />
  <br />
  <p><b>Una experiencia de Poker elegante y rápida para tu escritorio, construida con la potencia de Rust y la agilidad de Tauri.</b></p>
</div>

---

<div align="center">

[![Rust](https://img.shields.io/badge/rust-%23E32F26.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![React](https://img.shields.io/badge/react-%2320232a.svg?style=for-the-badge&logo=react&logoColor=%2361DAFB)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Vite](https://img.shields.io/badge/vite-%23646CFF.svg?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)

</div>

---

## ✨ Características

-   🚀 **Rendimiento Nativo:** Gracias a Rust, el motor de juego es increíblemente rápido y eficiente.
-   🎨 **Interfaz Moderna:** Desarrollada con React para una experiencia de usuario fluida y atractiva.
-   💻 **Multiplataforma:** Disponible para Windows, macOS y Linux.
-   🔒 **Seguridad:** Tauri garantiza que tu aplicación sea segura por defecto con una superficie de ataque mínima.

## 🚀 Próximamente (Roadmap)

-   [ ] 🃏 Lógica completa de Texas Hold'em.
-   [ ] 💰 Sistema de apuestas y gestión de fichas.
-   [ ] 🌐 Multijugador local y online.
-   [ ] 🎨 Gráficos de cartas personalizados.
-   [ ] 📊 Estadísticas detalladas del jugador.

## 🛠️ Requisitos Previos

Antes de comenzar, asegúrate de tener instalado:

-   [Rust](https://www.rust-lang.org/tools/install) (última versión estable)
-   [Node.js](https://nodejs.org/) (recomendado v18 o superior)
-   Dependencias del sistema para Tauri (revisa la [guía oficial](https://tauri.app/v1/guides/getting-started/prerequisites) según tu SO)

## 📦 Instalación y Desarrollo

1.  **Clonar el repositorio:**
    ```bash
    git clone https://github.com/tu-usuario/mpoker.git
    cd mpoker
    ```

2.  **Instalar dependencias de Node:**
    ```bash
    npm install
    ```

3.  **Iniciar el entorno de desarrollo:**
    ```bash
    npm run tauri dev
    ```

4.  **Compilar para producción:**
    ```bash
    npm run tauri build
    ```

## 📂 Estructura del Proyecto

-   `src/`: El corazón del frontend en React y TypeScript.
-   `src-tauri/`: El backend en Rust que maneja la lógica pesada y el sistema.
-   `src-tauri/src/`: Comandos de Tauri y lógica de juego en Rust.
-   `public/`: Assets estáticos como iconos y fuentes.

## 🤝 Contribuciones

¡Las contribuciones son las que hacen que la comunidad de código abierto sea un lugar increíble! Cualquier contribución que hagas será **muy apreciada**.

1. Haz un Fork del proyecto
2. Crea tu Rama de Característica (`git checkout -b feature/AmazingFeature`)
3. Haz un Commit de tus Cambios (`git commit -m 'Add some AmazingFeature'`)
4. Haz un Push a la Rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

---

<div align="center">
  Hecho con ❤️ para los amantes del Poker y el buen código.
</div>
