# A Windows window manager for power users

The goal of this application is to allow users to manipulate and manage windows _without_ the mouse.

Fenestra operates on monitors and windows, but also implements the concept of _workspaces_ (akin to Windows Virtual
Desktops).<br>
Each monitor contains 10 workspaces, and each workspace contains any number of windows.<br>

As a visual aid, Fenestra was designed with the following hierarchy in mind:

- Monitors
    - Workspaces
        - Windows

# Planned Features

## Windows

- Focus window in direction ✅
- Move window in direction ✅
- Close window ✅
- Dynamic window sizing ✅
- Active window border
- Window gaps

## Workspaces

- Multiple workspaces ✅
- Focus workspace ✅
- Send window to workspace ✅
- Workspaces per monitor ✅
- Support multiple split axes simultaneously (currently only horizontal OR vertical based on monitor dimensions)

## UX

- Binary downloads
- Config GUI
- Cloud config storage
- Config per machine
- Improved registry edits

# Configuration

Fenestra is configured by writing entries into the configuration file `fenestra.conf` located in this directory.<br>
If the configuration file does not exist, a blank default will be generated.

## Available Actions and their associated identifiers

### Window Actions:

These actions interact with windows/applications

#### Focus Window in Direction:

- `focus_window_left` Focus the window left of the current one
- `focus_window_down` Focus the window below the current one
- `focus_window_up` Focus the window above the current one
- `focus_window_right` Focus the window right of the current one

Note: If no adjacent window in the specified direction exists, these commands fallback to an adjacent monitor in that
direction, and failing that, do nothing

#### Move Window in Direction:

- `move_window_left` Move the current window left
- `move_window_down` Move the current window down
- `move_window_up` Move the current window up
- `move_window_right` Move the current window right

#### Misc Window Commands:

- `close_window` Close the current window

### Workspace Actions:

#### Focus Workspace:

- `focus_workspace_1` Focus workspace 1
- `focus_workspace_2` Focus workspace 2
- ...
- `focus_workspace_9` Focus workspace 9
- `focus_workspace_0` Focus workspace 10

#### Send to Workspace:

- `send_to_workspace_1` Send the current window to workspace 1
- `send_to_workspace_2` Send the current window to workspace 2
- ...
- `send_to_workspace_9` Send the current window to workspace 9
- `send_to_workspace_0` Send the current window to workspace 10

## Format

Fenestra configuration entries should follow the format: `identifier: value`.<br>
In other words, config entries are made up of a single line containing two strings, **identifier** and **value**,
separated by a colon.

**Identifiers** may be any valid action identifier (more on those later), or a variable identifier.<br>
**Values** must be a valid sequence of key names, each separated by a plus, and may contain variables.

### Variables

Using variables can make maintaining a config much easier.<br>
Variables may be defined via a config entry, then referenced throughout any following entries.<br>
Variables are defined by creating a config entry using the desired variable name as the identifier, and the desired key(
s) as the value.<br>
Variables may be referenced inside the value portion of a config entry by prefixing the variable name with a
dollar-sign.<br>

**Note**: Config variables operate with five implicit assumptions

- All variable definitions are referenced at least once
- Each variable definition is unique
- Variable declarations precede their respective references
- While variable values may reference another variable, the resulting value *eventually* resolves to a valid key.
- There are no cyclic variable references

#### Example: Creating a 'prefix' variable

Using a common key (or set of keys) as a prefix for Fenestra keybinds can be useful when attempting to avoid
pre-existing keyboard shortcuts.<br>

To do so, first declare a variable named `prefix`, and set the value to be the desired key(s): <br>
`prefix: WIN`

Now reference the variable in later config entries like so:<br>
`focus_left: $prefix + h`
`focus_right: $prefix + l`

This would later resolve to:
`focus_left: WIN + h`
`focus_right: WIN + l`

## A note on binding the Windows key

When possible, Fenestra will intercept and suppress key events that contain the Windows (WIN) key.

It's worth noting however, that many shortcuts using the WIN key have been pre-defined by Microsoft.

Those shortcuts operate on three distinct levels, with each level broadening in scope.

1. Application level
    - Shortcuts relevant to a specific application (ex. close window, new tab, minimize/maximize window)
    - Require the application to be the currently active window
2. Operating System (OS) level
    - Shortcuts relevant to the operating system (ex. opening the start menu by pressing the Windows key)
    - Take precedence over application level shortcuts
    - Execute regardless of the currently active window
3. Kernel level
    - Shortcuts relevant to the system kernel (ex. locking the desktop)
    - Take precedence over application AND operating system level shortcuts
    - Execute regardless of the current state of the system

OS level shortcuts include:

- `WIN + R` (Run dialog)
- `WIN + E` (File Explorer)
- `WIN + N` (Notifications panel)
- `WIN + K` (Casting panel)
- `WIN + W` (Widgets panel)
- `WIN + U` (Accessibility panel)
- `WIN + H` (Speech to text)

... and many others

Kernel level shortcuts include:

- `WIN + L` (Lock the desktop)
- `WIN + G` (Xbox GameBar)

Although Fenestra can intercept application and OS level shortcuts, it is (at least, to my knowledge), **impossible** to
intercept kernel level
shortcuts.

As a work-around, kernel level shortcuts may be disabled.

### Disabling OS level shortcuts

While not necessary, it is possible to disable all OS level shortcuts by creating one registry key:

- Open the Registry Editor
- In the address bar, enter `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies`
- In the right pane, right-click any empty space and select `New` > `Key`
- Name the new key `System`
- In the right pane, right-click any empty space and select `New` > `DWORD (32-bit) Value`
- Set the value name to `NoWinKeys`, and the value data to `1`
- Click `OK`

### Disabling kernel level shortcuts

Because kernel level shortcuts are technically impossible to disable, 'disabling' a
kernel level shortcut usually entails disabling whatever the shortcut _executes_.

#### Win + L

Note: The steps below actually disable locking the PC altogether, not just the shortcut

- Open the Registry Editor
- In the address bar, enter `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon`
- In the right pane, double-click `DisableLockWorkStation`, and set the value data to `1`
- Click `OK`

#### Win + G

Note: The steps below actually uninstall Xbox GameBar, not just the shortcut

- Open Powershell as Administrator
-

Run `Get-AppxPackage -AllUsers -PackageTypeFilter Bundle -Name "*Microsoft.XboxGamingOverlay*" | Remove-AppxPackage -AllUser`
