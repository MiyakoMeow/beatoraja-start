# 另一个 beatoraja 启动器

- 请点击 [Releases](https://github.com/MiyakoMeow/beatoraja-start/releases) 查阅最新下载。

1. 从 [该网站](https://mocha-repository.info/download.php) 下载 `beatoraja`，请务必下载 `-jre-win64` 版本。
2. 从 [Releases](https://github.com/MiyakoMeow/beatoraja-start/releases/latest) 下载 `beatoraja-start.exe`。
3. 将下载好的 `beatoraja-start.exe` 复制或移动到与 `beatoraja.jar` 相同的目录中。
4. 运行 `beatoraja-start.exe`。

如果出现问题，用相同的方式放置`beatoraja-start-with-log.exe`并运行，查看日志以排查问题。

## 参考启动命令

```powershell
.\jre\bin\javaw.exe -Xms1g -Xmx8g -XX:+UseZGC -XX:+DisableExplicitGC -XX:+TieredCompilation -XX:+UseNUMA -XX:+AlwaysPreTouch -XX:-UsePerfData -XX:+UseThreadPriorities -XX:+ShowCodeDetailsInExceptionMessages -cp "beatoraja.jar;ir/*" bms.player.beatoraja.MainLoader
```
