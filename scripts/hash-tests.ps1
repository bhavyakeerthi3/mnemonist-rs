# Hash original test files for Port Mortem kickoff verification.
$files = Get-ChildItem -Path "$PSScriptRoot\..\tests\original\*.js" | Sort-Object Name
$hashes = @()
foreach ($file in $files) {
    $hash = (Get-FileHash $file.FullName -Algorithm SHA256).Hash
    $hashes += "$($file.Name) $hash"
    Write-Output "$($file.Name) $hash"
}
$combined = ($hashes -join "`n")
$manifestHash = (Get-FileHash -InputStream ([IO.MemoryStream]::new([Text.Encoding]::UTF8.GetBytes($combined))) -Algorithm SHA256).Hash
Write-Output ""
Write-Output "MANIFEST_SHA256 $manifestHash"
