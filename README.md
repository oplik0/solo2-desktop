# Solo2 Desktop App

An unofficial desktop application for managing and interacting with your SoloKeys Solo v2

## Features

- [x] List of plugged in keys with basic information about them (UUID, version, secure/hacker)
- [x] Updating keys
- [x] Automatically checking for new key updates (currently the app sends a request every 15 minutes)
- [x] Listing and managing TOTP credentials stored on keys
- [x] Generating TOTP codes
- [x] Basic device diagnostics (wink, reboot to bootloader)
- [ ] HOTP
- [ ] Managing Discoverable Credentials
- [ ] Change pin
- [ ] Upload firmware from a file
- [ ] Actually decent UX

## Developing

Once you cloned the repository install dependencies with ``pnpm install`. Then to start a dev application instance run:

```bash
pnpm tauri dev

# or start just the frontend development server
pnpm dev
```

## Building

To build a production version of the app:

```bash
pnpm tauri build

# or to only build the Svelte app
pnpm build
```

## License

The code in this repository is under [MIT License](LICENSE).
This application is using icons from [Microsoft's Fluent System Icons](https://github.com/microsoft/fluentui-system-icons) which are licensed under [MIT](static/licenses/fluentui-icons-license)
