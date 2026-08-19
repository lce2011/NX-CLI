# NX-CLI

### Command
`nx-cli <option> <flags>`

### Subcommands & Options
**new** `nx-cli new -e <empty> -l <lang> -n <name>`

*Generates a new template project.*

| Options    | Type | Values      | Necessary |
|:------------:|:------:|:-------------:|:-----------:|
| `-e/--empty` | bool | `true, false` | No        |
| `-l/--lang`  | &str | `c, cpp, cxx` | No        |
| `-n/--name`  | &str | `<name>`      | Yes       |

**update** `nx-cli update`

*Updates devkitPro, devkitA64 and libnx.*

> [!WARNING]
> The update subcommand doesn't work on UNIX-like Systems like Linux and MacOS yet.
