# ⚠️ WARNING ⚠️
This is not the official repository and I don't intend to maintain it. I'm using it for personal purposes until the official repository fixes issues with importing and exporting worlds (see issue [#14](https://github.com/salpland/haze/issues/14)).

# ☘️ Haze

> A simple command line tool to manage your Minecraft Bedrock worlds

Haze allows you to keep your project's worlds out of the `com.mojang` directory and place them in your project's directory instead. This way you can easily work on multiple worlds and move them back and forth between `com.mojang` and your project's directory.

![Diagram](.github/diagram.png)

## 📦 Installation

Open PowerShell on Windows and run:

⚠️ WARNING ⚠️: This command installs the unofficial version of Haze. If you want to install the official version, please refer to the [official repository](https://github.com/salpland/haze)

```powershell
irm https://raw.githubusercontent.com/nusiq/haze/main/scripts/install.ps1 | iex
```

**You can also use this same command to update Haze.**

## 🧩 Usage

Haze requires your project to include a config file that follows the [Project Config Standard](https://github.com/Bedrock-OSS/project-config-standard).

This also means that you can integrate Haze into projects that use [Regolith](https://github.com/Bedrock-OSS/regolith) or [bridge.'s Dash compiler](https://github.com/bridge-core/deno-dash-compiler) seamlessly.

### 🗺️ Setting up worlds

Here is the required config:

```jsonc
{
  // Now any world inside the "worlds" directory can be used in the command line argument.
  "worlds": ["./worlds/*"],
}
```

You can also reference multiple directories that store worlds:

```jsonc
{
  "worlds": ["./worlds/dev/*", "./worlds/demo/*"],
}
```

### 🖥️ Running commands

Run `haze --help` or reference the docs below:

| Command | Description |
| ------- | ----------- |
| `haze export <NAME>` | Copy a world from the project's worlds directory to "minecraftWorlds" |
| `haze export --overwrite <NAME>` | Overwrites if a world with the same name is already in "minecraftWorlds" |
| `haze export --path [stable, preview, education, <CUSTOM>]` | Predefined or custom export path |
| `haze import <NAME>` | Copy a world from "minecraftWorlds" to the project's worlds directory |
| `haze import --path [stable, preview, education, <CUSTOM>] <NAME>` | Predefined or custom import path |
| `haze list` | Lists the available worlds in the project config |

Note: `<NAME>` is the world directory name.

## 📝 License

Haze is under the MIT license.
