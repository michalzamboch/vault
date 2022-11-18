$binaryFolders = 
    "..\target",
    "..\src\app_controller\target",
    "..\src\app_model\target",
    "..\src\app_view\target",
    "..\src\app_common\target",
    "..\src\app_tests\target"

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