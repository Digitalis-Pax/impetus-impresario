# Configuring the Impetus Impressario Server

Impetus Impressario is designed to manage settings by reading from multiple sources in a structured manner. This system ensures that configuration settings are layered and prioritized, with later configurations overriding earlier ones.

## Sources

Each source in this list overrides the ones that come before it. Missing sources are ignored.

1. The path `config/configuration.json` in the same folder as the application
2. The Linux/Posix system wide configuration at `/etc/impetus-impressario/configuration.json`
3. The configuration for the user it is running as
	- On Linux/Posix systems: `$HOME/.config/impetus-iimpressario/configuration.json`
	- On Windows systems:
		+ The Roaming AppData folder: `C:\Users\$USER\AppData\Roaming\impetus-impresario\configuration.json`
		+ The Local AppData folder: `C:\Users\$USER\AppData\Local\impetus-impresario\configuration.json`
	- On MacOS systems:
		+ The Application Support folder: `$HOME/Library/Application Support/impetus-impresario/configuration.json`
		+ The User Preferences folder: `$HOME/Library/Preferences/impetus-impressario/configuration.json`
4. Environment variables prefixed with `II_`: `II_DATASTORE_HOST=localhost`
5. Command line arguments: `impetus-impressario --datastore.host=localhost`
