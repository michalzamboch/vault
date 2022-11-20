$binaryFolders = 
    "..\target",
    "..\src\app_controller\target",
    "..\src\app_model\target",
    "..\src\app_view\target",
    "..\src\app_common\target",
    "..\src\app_tests\target"

$lockFiles = 
    "..\Cargo.lock",
    "..\src\app_controller\Cargo.lock",
    "..\src\app_model\Cargo.lock",
    "..\src\app_view\Cargo.lock",
    "..\src\app_common\Cargo.lock",
    "..\src\app_tests\Cargo.lock"

foreach ($folder in $binaryFolders)
{
    if ( Test-Path -Path $folder )
    {
        Write-Host "Deleting binaries: $folder"
        Remove-Item $folder -Recurse
    }
    else
    {
        Write-Host "Already cleaned: $folder"
    }
}

Write-Host "Binaries cleaned..."


foreach ($file in $lockFiles)
{
    if ( Test-Path -Path $file )
    {
        Write-Host "Deleting file: $file"
        Remove-Item $file
    }
    else
    {
        Write-Host "Already cleaned: $file"
    }
}

Write-Host "Lock files cleaned..."