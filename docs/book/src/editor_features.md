# Editor Features

## VS Code

### Color configurations

It is possible to change the foreground/background color and font family/size of inlay hints.
Just add this to your `settings.json`:

```json
{
  "editor.inlayHints.fontFamily": "Courier New",
  "editor.inlayHints.fontSize": 11,

  "workbench.colorCustomizations": {
    // Name of the theme you are currently using
    "[Default Dark+]": {
      "editorInlayHint.foreground": "#868686f0",
      "editorInlayHint.background": "#3d3d3d48",

      // Overrides for specific kinds of inlay hints
      "editorInlayHint.typeForeground": "#fdb6fdf0",
      "editorInlayHint.parameterForeground": "#fdb6fdf0",
    }
  }
}
```

### Semantic style customizations

You can customize the look of different semantic elements in the source code.
For example, mutable bindings are underlined by default, and you can override this behavior by adding the following section to your `settings.json`:

```json
{
  "editor.semanticTokenColorCustomizations": {
    "rules": {
      "*.mutable": {
        "fontStyle": "" // underline is the default
      }
    }
  }
}
```

Most themes do not support styling unsafe operations differently yet.
You can fix this by adding overrides for the rules `operator.unsafe`, `function.unsafe`, and `method.unsafe`:

```json
{
  "editor.semanticTokenColorCustomizations": {
    "rules": {
      "operator.unsafe": "#ff6600",
      "function.unsafe": "#ff6600",
      "method.unsafe": "#ff6600"
    }
  }
}
```

In addition to the top-level rules, you can specify overrides for specific themes.
For example, if you wanted to use a darker text color on a specific light theme, you might write:

```json
{
  "editor.semanticTokenColorCustomizations": {
    "rules": {
      "operator.unsafe": "#ff6600"
    },
    "[Ayu Light]": {
      "rules": {
        "operator.unsafe": "#572300"
      }
    }
  }
}
```

Make sure you include the brackets around the theme name.
For example, use `"[Ayu Light]"` to customize the theme Ayu Light.

### Special `when` clause context for keybindings

You may use the `inWgslProject` context to configure keybindings for WGSL projects only.
For example:

```json
{
  "key": "ctrl+alt+d",
  "command": "wgsl-analyzer.openDocs",
  "when": "inWgslProject"
}
```

More about `when` clause contexts [here](https://code.visualstudio.com/docs/getstarted/keybindings#_when-clause-contexts).

### Setting runnable environment variables

You can use the `wgsl-analyzer.runnables.extraEnv` setting to define runnable environment-specific substitution variables.
The simplest way for all runnables in a bunch:

```json
"wgsl-analyzer.runnables.extraEnv": {
  "RUN_SLOW_TESTS": "1"
}
```

Or it is possible to specify vars more granularly:

```json
"wgsl-analyzer.runnables.extraEnv": [
  {
    // "mask": null, // null mask means that this rule will be applied for all runnables
    "env": {
      "APP_ID": "1",
      "APP_DATA": "asdf"
    }
  },
  {
    "mask": "test_name",
    "env": {
      "APP_ID": "2" // overwrites only APP_ID
    }
  }
]
```

You can use any valid regular expression as a mask.
Also, note that a full runnable name is something like **`run bin_or_example_name`**, **`test some::mod::test_name`**, or **`test-mod some::mod`**.
It is possible to distinguish binaries, single tests, and test modules with these masks: `"^run"`, `"^test "` (the trailing space matters!), and `"^test-mod"` respectively.

If needed, you can set different values for different platforms:

```json
"wgsl-analyzer.runnables.extraEnv": [
  {
    "platform": "win32", // windows only
    "env": {
      "APP_DATA": "windows specific data"
    }
  },
  {
    "platform": ["linux"],
    "env": {
      "APP_DATA": "linux data"
    }
  },
  { // for all platforms
    "env": {
      "APP_COMMON_DATA": "xxx"
    }
  }
]
```

### Compiler feedback from external commands

You can configure VS Code to run a command in the background and use the `$wgsl-watch` problem matcher to generate inline error markers from its output.
To do this, you need to create a new [VS Code Task](https://code.visualstudio.com/docs/editor/tasks) and set `"wgsl-analyzer.checkOnSave": false` in preferences.
Example `.vscode/tasks.json`:

```json
{
  "label": "Watch",
  "group": "build",
  "type": "shell",
  "command": "example-tool watch",
  "problemMatcher": "$wgsl-watch",
  "isBackground": true
}
```

### Live Share

VS Code Live Share has partial support for `wgsl-analyzer`.

Live Share *requires* the official Microsoft build of VS Code; OSS builds will not work correctly.

The host's `wgsl-analyzer` instance will be shared with all guests joining the session.
The guests do not have to have the `wgsl-analyzer` extension installed for this to work.

If you are joining a Live Share session and *do* have `wgsl-analyzer` installed locally, then commands from the command palette will not work correctly.
This is because they will attempt to communicate with the *local* server, not the server of the session host.
