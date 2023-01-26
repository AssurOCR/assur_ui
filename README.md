# assur_ui: rapid + easy native GUI development in Rust
Welcome to **assur_ui**, an ECS pattern native GUI library written in Rust. assur_ui is built on top of wxWidgets, a mature and stable cross-platform framework that provides native UI on Windows, Mac, FreeBSD, and Linux.

With assur_ui, you can easily create powerful and elegant user interfaces for your Rust-based applications. Our library follows the ECS (Entity-Component-System) pattern, which makes it easy to organize and manage your UI elements and their behavior. This pattern allows for separation of concerns and makes it easy to reason about the structure and behavior of your application.

assur_ui is designed to be lightweight and easy to use, with a simple ECS-based API. Our library also comes with a wide range of built-in components and controls, including buttons, labels, text boxes, list boxes, and more. And if you need more advanced functionality, you can easily create your own custom components using the provided API.

# Goals
-   Easiest possible GUI library
-   Native GUI on Windows, Macos, FreeBSD and Linux (Both GTK and Qt {under-development}
-   ECS-styled work flow
-   Both native and bevy powered API for painting, while not compromising with native look and feel.
-   Support XML based descriptor for Component creation
- Keep adding support for more desktop and mobile platforms
## Non-goals
-   Paint all widgets ourselves
-   Support web-browser