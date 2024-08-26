# 🔐 sendenv - 1Password `.env` Sharer 🚀

📤 Easily upload and share files as secure notes in 1Password!

## 🌟 Features

- 📁 Upload any file as a secure note to 1Password
- 🔗 Generate shareable links
- 📋 Automatically copy share links to clipboard
- 📧 Optional email-gated sharing
- 🔒 One-time view option for extra security

## 🛠️ Installation

1. Ensure you have Rust installed
2. Clone this repository
3. Run `cargo build --release`

## 🚀 Usage

```bash
sendenv [OPTIONS] --path <PATH_TO_FILE>
```

### 🎛️ Options

- `-p, --path <PATH_TO_FILE>`: Path to the file you want to share (default: `.env` of current working directory)
- `-e, --email <EMAIL>`: Require email to access the shared note
- `-d, --display-once`: Make the link expire after a single use

## 🔑 Prerequisites

- 1Password CLI installed and configured
- Active 1Password session (`eval $(op signin)`)

## 📚 Dependencies

- 🤖 Clap - a tool that simplifies the creation of CLI applications
- 📋 Arboard - a tool that streamlines clipboard management

## 🛡️ Security

This tool uses 1Password's secure infrastructure to protect your sensitive data.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.

## 🧑‍💻 Developer

Developed by Julian Grande

---

💡 Need help? Open an issue or contact [me via email](mailto:juliangr@stud.ntnu.no)
