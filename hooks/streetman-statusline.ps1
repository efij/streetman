$root = if ($env:PLUGIN_DATA) { $env:PLUGIN_DATA } else { Join-Path $HOME ".streetman" }
$flag = Join-Path $root ".streetman-lean-active"
if (Test-Path $flag) {
  $mode = (Get-Content $flag -TotalCount 1).Trim().ToUpperInvariant()
  if ($mode) { Write-Output "[STREET:LEAN:$mode]" }
}
