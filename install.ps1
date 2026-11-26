$ErrorActionPreference = "Stop"

$Repo = if ($env:ORGAZ_REPO) { $env:ORGAZ_REPO } else { "Sash1ro/file-organizer" }
$Version = if ($env:ORGAZ_VERSION) { $env:ORGAZ_VERSION } else { "latest" }
$InstallDir = if ($env:ORGAZ_INSTALL_DIR) { $env:ORGAZ_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA "orgaz\bin" }

$Archive = "orgaz-x86_64-pc-windows-msvc.zip"
if ($Version -eq "latest") {
    $Url = "https://github.com/$Repo/releases/latest/download/$Archive"
} else {
    $Tag = if ($Version.StartsWith("v")) { $Version } else { "v$Version" }
    $Url = "https://github.com/$Repo/releases/download/$Tag/$Archive"
}

$TmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ("orgaz_" + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

try {
    $ZipPath = Join-Path $TmpDir $Archive
    Write-Host "Downloading $Url..."
    Invoke-WebRequest -Uri $Url -OutFile $ZipPath
    Expand-Archive -Path $ZipPath -DestinationPath $TmpDir -Force

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item (Join-Path $TmpDir "orgaz.exe") (Join-Path $InstallDir "orgaz.exe") -Force
    Write-Host "Installed orgaz to $(Join-Path $InstallDir 'orgaz.exe')"

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathParts = if ($UserPath) { $UserPath -split ";" } else { @() }
    if ($PathParts -notcontains $InstallDir) {
        $NewPath = if ($UserPath) { "$UserPath;$InstallDir" } else { $InstallDir }
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        Write-Host "Added $InstallDir to your User PATH. Restart your terminal to use 'orgaz'."
    } else {
        Write-Host "orgaz is already in your PATH. Run: orgaz --help"
    }
} finally {
    Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
}
