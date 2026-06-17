# streetman bootstrap (Windows): make the plugin zero-prerequisite.
# Runs on SessionStart. No-op if the binary is reachable; otherwise installs one
# (prebuilt release, else cargo). Never blocks or fails the session.

$ErrorActionPreference = 'SilentlyContinue'

$home_dir = if ($env:STREETMAN_HOME) { $env:STREETMAN_HOME } else { Join-Path $env:USERPROFILE '.streetman' }
$bin_dir = Join-Path $home_dir 'bin'
$bin = Join-Path $bin_dir 'streetman.exe'
$repo = 'efij/streetman'

if (Get-Command streetman -ErrorAction SilentlyContinue) { exit 0 }
if (Test-Path $bin) { Set-Content -Path (Join-Path $home_dir 'bin-path') -Value $bin_dir; exit 0 }

New-Item -ItemType Directory -Force -Path $bin_dir | Out-Null

# 1) Prefer prebuilt release binary.
$url = "https://github.com/$repo/releases/latest/download/streetman-windows-x64.exe"
try { Invoke-WebRequest -Uri $url -OutFile $bin -UseBasicParsing } catch {}

# 2) Fall back to cargo build.
if (-not (Test-Path $bin) -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
  Write-Host 'streetman: building from source (one-time)...'
  cargo install --git "https://github.com/$repo" streetman-cli --bin streetman --locked --root $home_dir 2>$null | Out-Null
}

if ((Test-Path $bin) -or (Get-Command streetman -ErrorAction SilentlyContinue)) {
  Set-Content -Path (Join-Path $home_dir 'bin-path') -Value $bin_dir
  Write-Host 'streetman: ready.'
} else {
  Write-Host 'streetman: could not auto-install. Install once with:'
  Write-Host "  cargo install --git https://github.com/$repo streetman-cli --bin streetman --locked"
}

exit 0
