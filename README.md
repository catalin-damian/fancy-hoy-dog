# 🐶 Fancy Hoy Dog

**Fancy Hoy Dog** is a sleek, cross-platform Rust application built with [Dioxus](https://dioxuslabs.com/) that lets users browse, view, and save their favorite dog images. It supports both web and desktop targets and uses SQLite for persistence.

---

## 🚀 Features

- 🐕 Random dog image viewer
- ❤️ Save your favorite dogs
- 🧭 Clean navigation
- 🎯 Cross-platform: Web & Desktop
- 💾 Local SQLite database support (desktop mode)
- 🎨 Responsive UI with custom styling (CSS)

---

## 🛠 Tech Stack

- **Rust** with [Dioxus](https://github.com/DioxusLabs/dioxus)
- **SQLite** via [`rusqlite`](https://docs.rs/rusqlite)
- **reqwest** for HTTP requests
- **serde** for JSON serialization
- **Custom CSS** for styling

---

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.4/getting_started/installation/)

```bash
cargo install dioxus-cli

🧪 Running the App

Web
dx serve

Desktop
dx serve --platform desktop

📁 Project Structure
hot_dog/
├── assets/              # Static CSS
├── src/
│   ├── components/      # UI components
│   ├── main.rs          # App entry point
│   ├── backend.rs       # SQLite integration
│   ├── favorites.rs     # Favorites logic
│   ├── nav.rs, view.rs  # UI views
├── hotdog.db            # SQLite database (desktop only)
├── Cargo.toml
├── Dioxus.toml
└── README.md

📄 License

This project is licensed under the MIT License.

🙌 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to open a pull request.

🌟 Acknowledgments

Thanks to Dioxus for enabling elegant Rust-based UIs.
Powered by the Dog CEO API

---

Let me know if you'd like to generate a matching `LICENSE` file or add CI support with GitHub Actions!
