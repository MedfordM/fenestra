# A note on binding the Windows key

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

## Disabling OS level shortcuts

While not necessary, it is possible to disable all OS level shortcuts by creating one registry key:

- Open the Registry Editor
- In the address bar, enter `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies`
- In the right pane, right-click any empty space and select `New` > `Key`
- Name the new key `System`
- In the right pane, right-click any empty space and select `New` > `DWORD (32-bit) Value`
- Set the value name to `NoWinKeys`, and the value data to `1`
- Click `OK`

## Disabling kernel level shortcuts

Because kernel level shortcuts are technically impossible to disable, 'disabling' a
kernel level shortcut usually entails disabling whatever the shortcut _executes_.

#### Win + L

Note: The steps below actually disable locking the PC altogether, not just the shortcut

- Open the Registry Editor
- In the address bar, enter `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon`
- In the right pane, double-click `DisableLockWorkStation`, and set the value data to `1`
- Click `OK`

### Win + G

Note: The steps below actually uninstall Xbox GameBar, not just the shortcut

- Open Powershell as Administrator
- Run `Get-AppxPackage -AllUsers -PackageTypeFilter Bundle -Name "*Microsoft.XboxGamingOverlay*" | Remove-AppxPackage -AllUser`
