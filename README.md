# ttp
Telegram .attheme file parser written in Rust.

```
USAGE:
    ttp [OPTIONS] <theme>

ARGS:
    <theme>    Path to the .attheme (or .ttp if used with -r) file

OPTIONS:
    -h, --help       Print help information
    -r, --revert     Use this flag to reverse parsing of a theme. You have to provide a .ttp file in
                     <theme>
    -s, --sort       Use this flag to sort lines in alphabetical order
    -V, --version    Print version information
```

**Examples**

```
$ ttp my-cool-theme.attheme
$ head -n5 my-cool-theme.attheme my-cool-theme.ttp
==> my-cool-theme.attheme <==
actionBarActionModeDefault=-16777216
actionBarActionModeDefaultIcon=-1
actionBarActionModeDefaultSelector=-13092802
actionBarActionModeDefaultTop=268435456
actionBarBrowser=-16777216

==> my-cool-theme.ttp <==
actionBarActionModeDefault: #FF000000
actionBarActionModeDefaultIcon: #FFFFFFFF
actionBarActionModeDefaultSelector: #FF38383E
actionBarActionModeDefaultTop: #10000000
actionBarBrowser: #FF000000

```
To create a file in .attheme format from a .ttp file use this command 
```
$ ttp -r my-cool-theme.ttp
```
