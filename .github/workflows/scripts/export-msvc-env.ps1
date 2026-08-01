$ErrorActionPreference = "Stop"

$vswhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
$installationPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $installationPath) {
    throw "Unable to locate an x64 MSVC installation"
}

$developerCommand = Join-Path $installationPath "Common7\Tools\VsDevCmd.bat"
$environmentLines = & cmd.exe /s /c "`"$developerCommand`" -no_logo -arch=x64 -host_arch=x64 >nul && set"
if ($LASTEXITCODE -ne 0) {
    throw "VsDevCmd.bat failed with exit code $LASTEXITCODE"
}

$developerEnvironment = @{}
foreach ($line in $environmentLines) {
    if ($line -match '^([^=]+)=(.*)$') {
        $developerEnvironment[$matches[1]] = $matches[2]
    }
}

$requiredVariables = @(
    "INCLUDE",
    "LIB",
    "LIBPATH",
    "Path",
    "VCINSTALLDIR",
    "VCToolsInstallDir",
    "WindowsSdkDir"
)
$optionalVariables = @(
    "UCRTVersion",
    "UniversalCRTSdkDir",
    "WindowsLibPath",
    "WindowsSdkBinPath",
    "WindowsSDKLibVersion",
    "WindowsSDKVersion"
)

$utf8 = [System.Text.UTF8Encoding]::new($false)
foreach ($name in $requiredVariables) {
    if (-not $developerEnvironment.ContainsKey($name)) {
        throw "MSVC developer environment did not define $name"
    }
    [System.IO.File]::AppendAllText($env:GITHUB_ENV, "$name=$($developerEnvironment[$name])`n", $utf8)
}
foreach ($name in $optionalVariables) {
    if ($developerEnvironment.ContainsKey($name)) {
        [System.IO.File]::AppendAllText($env:GITHUB_ENV, "$name=$($developerEnvironment[$name])`n", $utf8)
    }
}

$gitBash = Join-Path $env:ProgramFiles "Git\usr\bin\bash.exe"
if (-not (Test-Path $gitBash)) {
    throw "Unable to locate Git Bash at $gitBash"
}

[System.IO.File]::AppendAllText($env:GITHUB_ENV, "BAZEL_SH=$gitBash`n", $utf8)
[System.IO.File]::AppendAllText($env:GITHUB_ENV, "BAZEL_VC=$($developerEnvironment['VCINSTALLDIR'])`n", $utf8)
