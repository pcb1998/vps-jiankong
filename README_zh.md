[English](README.md)

# 我的应用场景

数月前，我收到了VPS提供商的一封邮件，通知我由于我的VPS长期CPU占用过高，影响了其他用户，因此将要关停我的服务器。当时，我完全不清楚是哪个程序导致了问题。

它的主要功能是：
  * 监控CPU使用率：作为一个后台服务运行，持续监控服务器的CPU占用情况。
  * 分级响应：
      * 当CPU占用率超过40%时，它会记录下当前占用最高的10个进程信息到系统日志中，方便排查问题。
      * 当CPU占用率超过80%时，它会发送邮件通知用户。
      * 当CPU占用率超过90%时，它会启动一个5分钟的关机倒计时。如果在此期间CPU占用率没有在1分钟内持续低于80%，它将自动执行关机命令，以防止服务器因长时间
        高负载而出现问题。

## 部署

要部署 `jiankong` 服务，请按照以下步骤操作：

1.  **放置可执行文件：**
    确保 `jiankong` 可执行文件位于 `/usr/local/bin/`。如果不在那里，请从当前目录复制它：
    ```bash
    sudo cp jiankong /usr/local/bin/
    ```

2.  **创建 systemd 服务文件：**
    在 `/etc/systemd/system/` 中创建一个名为 `jiankong.service` 的新 systemd 服务单元文件，内容如下：

    ```ini
    [Unit]
    Description=CPU Monitor Service
    After=network.target

    [Service]
    Type=simple
    User=root
    WorkingDirectory=/usr/local/bin/
    ExecStart=/usr/local/bin/jiankong
    Restart=on-failure
    RestartSec=5
    KillMode=process

    [Install]
    WantedBy=multi-user.target
    ```

    您可以使用您喜欢的文本编辑器创建此文件，例如：
    ```bash
    sudo nano /etc/systemd/system/jiankong.service
    ```
    然后粘贴上述内容并保存文件。

3.  **重新加载 systemd 并启动服务：**
    创建服务文件后，重新加载 systemd 守护程序并启动 `jiankong` 服务：
    ```bash
    sudo systemctl daemon-reload
    sudo systemctl start jiankong
    ```

4.  **开机启动服务（可选）：**
    为确保服务在系统重启后自动启动，请启用它：
    ```bash
    sudo systemctl enable jiankong
    ```

## 使用

服务运行后，它将按照配置监控 CPU 使用情况。您可以通过以下命令检查服务状态：
```bash
sudo systemctl status jiankong
```
