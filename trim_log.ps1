$filePath = ".\log.txt"
$outputFilePath = ".\trimmed_log.txt"
$trimmedLines = @()

Get-Content $filePath | ForEach-Object {
    $trimmedLine = $_.Substring(0, [Math]::Min(73, $_.Length))
    $trimmedLines += $trimmedLine
}

$trimmedLines | Out-File $outputFilePath