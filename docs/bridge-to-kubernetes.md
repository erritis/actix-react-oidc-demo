# Fix issues with kubernetes plugin for vscode

## If complaining about dotnet

Install the dotnet version the plugin is complaining about. Discussion [of this issue](https://github.com/microsoft/mindaro/issues/105)

On Linux, add environment variables at system startup to **~/.profile**

>       export DOTNET_ROOT="~/.dotnet"
>       export BRIDGE_BINARYUTILITYVERSION=v1

And in **~/.bash_profile**:

>      if [[ -f ~/.profile ]] && . ~/.profile

## If it crashes when trying to update dependencies

Discussion [of this issue](https://github.com/microsoft/mindaro/issues/32)

In folder:

On **Windows**: *%UserProfile%\AppData\Roaming\Code\User\globalStorage\mindaro.mindaro\file-downloader-downloads\binaries*

On **MacOS**: */Users/{your_username}/Library/Application Support/Code/User/globalStorage/mindaro.mindaro/file-downloader-downloads/binaries*

On **Linux**: */home/{your_username}/.config/Code/User/globalStorage/mindaro.mindaro/file-downloader-downloads/binaries*

Run:

> sudo chmod +x dsc
> 
> sudo chmod +x kubectl/{your_platform}/kubectl
> 
> sudo chmod +x EndpointManager/EndpointManager

I had a **bridge** folder instead of the last **binaries** folder.