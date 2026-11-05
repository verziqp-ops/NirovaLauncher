; NIROVA Launcher's Windows installer (Inno Setup 6), built by .github/workflows/launcher.yml:
;   iscc /DAppVersion=1.2.0 /DSource=path\to\nirova-launcher.exe /Odist packaging\windows\setup.iss
; It installs for this user only, without administrator rights, in the same place as install.ps1
; (%LOCALAPPDATA%\Programs\NIROVA), so either one updates the other's install and the
; launcher's own updates replace the same file. Uninstalling keeps the game, worlds and settings
; (%APPDATA%\nirova-launcher).

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif
#ifndef Source
  #define Source "..\..\launcher\target\release\nirova-launcher.exe"
#endif
#define AppName "NIROVA Launcher"
#define AppExe "nirova-launcher.exe"

[Setup]
AppId={{6F0C3B7E-4A1D-4C8B-9E52-3D7A1F2B8C90}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=verziqp-ops
AppPublisherURL=https://github.com/verziqp-ops/NirovaLauncher
AppSupportURL=https://github.com/verziqp-ops/NirovaLauncher/issues
AppUpdatesURL=https://github.com/verziqp-ops/NirovaLauncher/releases
DefaultDirName={localappdata}\Programs\NIROVA
DisableDirPage=yes
DisableProgramGroupPage=yes
DisableReadyPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputBaseFilename=NIROVA-Setup
SetupIconFile=..\icons\nirova.ico
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName}
WizardStyle=modern
Compression=lzma2/max
SolidCompression=yes
CloseApplications=force
DefaultLanguage=russian

[Languages]
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"


[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "{#Source}"; DestDir: "{app}"; DestName: "{#AppExe}"; Flags: ignoreversion

[Icons]
Name: "{userprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"; WorkingDir: "{app}"
Name: "{userdesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
