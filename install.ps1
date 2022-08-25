echo "Installing cee`r`n"

if ((Get-Command "cargo.exe" -ErrorAction SilentlyContinue) -eq $null) 
{ 
   Write-Host "error: cargo.exe either not installed or not found in your PATH environment variable."
   Read-Host "Press enter to exit"
   Exit
}

cargo build --release

echo "Creating folder: $HOME\cee-project-manager-bin...`r`n"
mkdir -p "$HOME\cee-project-manager-bin" > $null

echo "Moving executables to $HOME\cee-project-manager-bin"
Move-Item -Path ".\target\release\*.exe" -Destination "$HOME\cee-project-manager-bin\"

#add rust-coreutils-binaries to the PATH environment variable
echo "Adding $HOME\cee-project-manager-bin to PATH environment variable...`r`n"
[System.Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";" + $HOME+"\cee-project-manager-bin", [System.EnvironmentVariableTarget]::User)

echo "cee project manager is now installed! you may have to restart this window for changes to take place."