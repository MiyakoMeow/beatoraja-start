# Another beatoraja executable for Windows

- See [Releases](https://github.com/MiyakoMeow/beatoraja-start/releases) for downloads.

1. Download `beatoraja` from [this site](https://mocha-repository.info/download.php). Remember to download `-jre-win64` version.
2. Place `beatoraja-start.exe` from [Releases](https://github.com/MiyakoMeow/beatoraja-start/releases/latest) into the same directory of `beatoraja.jar`.
3. Run `beatoraja-start.exe`.

If any problem encounters, try using `beatoraja-start-with-log.exe` with the same usage for logs.

## Run command

```powershell
.\jre\bin\javaw.exe -Xms1g -Xmx8g -XX:+UseZGC -XX:+DisableExplicitGC -XX:+TieredCompilation -XX:+UseNUMA -XX:+AlwaysPreTouch -XX:-UsePerfData -XX:+UseThreadPriorities -XX:+ShowCodeDetailsInExceptionMessages -cp "beatoraja.jar;ir/*" bms.player.beatoraja.MainLoader
```
