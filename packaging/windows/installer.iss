; ISCC /DStageDir=... /DOutputDir=... /DAppVersion=... /DNumericVersion=... installer.iss
#ifndef StageDir
  #error StageDir must point to the staged Windows package
#endif
#ifndef AppVersion
  #error AppVersion is required
#endif

[Setup]
AppId={{5E92CF22-A4E7-4641-A065-59D1E60E8A90}
AppName=Borrow Fighters
AppVersion={#AppVersion}
VersionInfoVersion={#NumericVersion}
DefaultDirName={localappdata}\Programs\Borrow Fighters
DefaultGroupName=Borrow Fighters
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0.18362
OutputDir={#OutputDir}
OutputBaseFilename=borrow-fighters-{#AppVersion}-windows-x86_64-setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\borrow-fighters.exe
CloseApplications=yes

[Languages]
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Criar atalho na área de trabalho"; Flags: unchecked

[Files]
Source: "{#StageDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Borrow Fighters"; Filename: "{app}\borrow-fighters.exe"; WorkingDir: "{app}"
Name: "{group}\Como jogar"; Filename: "{sys}\notepad.exe"; Parameters: """{app}\JOGUE-PRIMEIRO.md"""
Name: "{userdesktop}\Borrow Fighters"; Filename: "{app}\borrow-fighters.exe"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\borrow-fighters.exe"; Description: "Jogar Borrow Fighters"; Flags: nowait postinstall skipifsilent; WorkingDir: "{app}"
