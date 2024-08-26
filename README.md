# 🔐 sendenv - 1Password `.env` Sharer 🚀

📤 Easily upload and share files as secure notes in 1Password!

## 🌟 Features

- 📁 Upload any file as a secure note to 1Password
- 🔗 Generate shareable links
- 📋 Automatically copy share links to clipboard
- 📧 Optional email-gated sharing
- 🔒 One-time view option for extra security

## 🛠️ Installation

Ensure you have Rust and cargo installed on your system.
You also need the 1Password CLI installed on your system.

```bash
# Install 1Password CLI using your package manager (apt, yum, alpine, brew)
brew install --cask 1password-cli

# Authenticate using the 1Password CLI
eval $(op login)

# Install the application
cargo install sendenv
```

Thats it! 🙌

## 🚀 Usage

```bash
sendenv [OPTIONS] --path <PATH_TO_FILE>
```

### 📝 Example usage

```bash
# Uploads and creates a link for the `.env` file in the current directory
sendenv

# Uploads and creates a link for file `.env.development` in the cwd
sendenv -p env.development

# Uploads and shares link that can only be displayed once
# and which requires email verification from "example@user.com"
sendenv --email=example@user.com --display-once
```

### 🎛️ Options

- `-p, --path <PATH_TO_FILE>`: Path to the file you want to share (default: `.env` of current working directory)
- `-e, --email <EMAIL>`: Require email to access the shared note
- `-d, --display-once`: Make the link expire after a single use

## 📚 Dependencies

- 🤖 Clap - a tool that simplifies the creation of CLI applications
- 📋 Arboard - a tool that streamlines clipboard management

## 🔑 Prerequisites

- 1Password CLI installed and configured
- Active 1Password session (`eval $(op signin)`)

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
