# Configuration

Fenestra is configured by writing entries into the configuration file `fenestra.conf` located in this directory. 
If the configuration file does not exist, a default will be generated.

## Format
Config entries are made up of a single line containing two strings, identifier and value, separated by a colon.

Example: `identifier: value`

**Identifiers** may be any valid Fenestra action, or a variable name.

**Values** must be a valid sequence of key names, each separated by a plus, and may contain variables.

### Variables

Variables may be defined via a config entry, then referenced throughout any following entries.

Variable declarations must precede their respective references.

A variable is defined in a config entry by setting the name as the identifier, and the desired value as the value.

Example: `var_name: WIN`

A variable may be referenced in a config entry value by the variable name prefixed by a dollar-sign

Example: `left: $var_name + H`

Both defining a variable that has no references, _and_ referencing a variable that has no definition are considered invalid.

## Binding the Windows key

When possible, Fenestra attempts to intercept and suppress key events that contain the Windows (WIN) key.

It's worth noting however, that many shortcuts using the WIN key have been predefined by Microsoft.

These preexisting shortcuts operate on three distinct levels, with higher levels encompassing a more global scope:
 1. Application level
    - Shortcuts relevant to a specific application (ex. close window, new tab, minimize/maximize window)
    - Require the application to be the currently active window
 2. Operating System (OS) level
    - Shortcuts relevant to the operating system (ex. opening the start menu)
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

 ... and many others

Kernel level shortcuts include:
 - `WIN + L` (Lock the desktop)
 - `WIN + G` (Xbox Gamebar)

Although Fenestra can intercept application and OS level shortcuts, it is impossible to intercept kernel level shortcuts.

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

Because shortcuts handled at the kernel level are technically impossible to disable, 'disabling' a 
kernel level shortcut usually entails disabling whatever functionality gets executed _by_ the shortcut.

#### Win + L

Note: The steps below actually disable locking the PC altogether, not just the shortcut
 - Open the Registry Editor
 - In the address bar, enter `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon`
 - In the right pane, double-click `DisableLockWorkStation`, and set the value data to `1`
 - Click `OK`

#### Win + G

Note: The steps below actually disable the Xbox Game Bar altogether, not just the shortcut
 - Open the Registry Editor
 - In the address bar, enter `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR`
 - In the right pane, right-click any empty space and select `New` > `DWORD (32-bit) Value`
 - Set the value name to `AppCaptureEnabled`, and the value data to `0`
 - Click `OK`
 - In the address bar, enter `HKEY_CURRENT_USER\System\GameConfigStore`
 - Double-click `GameDVR_Enabled`
 - Set the value data to `0`
 - Click `OK`
 - Open Powershell as Administrator
 - Run `Get-AppxPackage -AllUsers -PackageTypeFilter Bundle -Name "*Microsoft.XboxGamingOverlay*" | Remove-AppxPackage -AllUser`
