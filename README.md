# Another beatoraja executable for Windows

- See [Releases](https://github.com/MiyakoMeow/beatoraja-start/releases) for usages.
- 请点击[Releases](https://github.com/MiyakoMeow/beatoraja-start/releases)查阅使用方式。

## Run command

```powershell
.\jre\bin\javaw.exe -Xms1g -Xmx8g -XX:+UseZGC -XX:+DisableExplicitGC -XX:+TieredCompilation -XX:+UseNUMA -XX:+AlwaysPreTouch -XX:+UseLargePages -XX:-UsePerfData -XX:+UseThreadPriorities -XX:+ShowCodeDetailsInExceptionMessages -cp "beatoraja.jar;ir/*" bms.player.beatoraja.MainLoader
```
