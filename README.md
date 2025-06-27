[中文](README_zh.md)

# jiankong - CPU Monitor Service

`jiankong` is a background service that continuously monitors your server's CPU usage and takes predefined actions based on different thresholds.

## Motivation

This tool was created after receiving a notice from a VPS provider about a potential server suspension due to consistently high CPU usage that was impacting other users. At the time, it was unclear which process was causing the problem.

`jiankong` was built to solve this by monitoring CPU usage, identifying the source of high load, and automatically taking protective measures.

## Features

- **Continuous CPU Monitoring:** Runs as a background service to keep track of CPU load.
- **Tiered Response System:**
    - **> 40% CPU Usage:** Logs the top 10 CPU-consuming processes to the system log for later analysis.
    - **> 80% CPU Usage:** Sends an email notification to the administrator to warn about the high CPU load.
    - **> 90% CPU Usage:** Initiates a 5-minute shutdown countdown. The shutdown is aborted if the CPU usage drops and remains below 80% for a continuous minute. Otherwise, the system will automatically shut down to prevent issues from prolonged high load.

## Deployment

To deploy the `jiankong` service, follow these steps:

1.  **Place the Executable:**
    Ensure the `jiankong` executable is located in `/usr/local/bin/`. If it's not there, copy it from the project directory:
    ```bash
    sudo cp jiankong /usr/local/bin/
    ```

2.  **Create the systemd Service File:**
    Create a new systemd service unit file named `jiankong.service` in `/etc/systemd/system/` with the following content:

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

    You can create this file using your preferred text editor. For example, using `nano`:
    ```bash
    sudo nano /etc/systemd/system/jiankong.service
    ```
    Then, paste the content above and save the file.

3.  **Reload systemd and Start the Service:**
    After creating the service file, reload the systemd daemon and start the `jiankong` service:
    ```bash
    sudo systemctl daemon-reload
    sudo systemctl start jiankong
    ```

4.  **Enable Service on Boot (Optional):**
    To ensure the service starts automatically after a system reboot, enable it:
    ```bash
    sudo systemctl enable jiankong
    ```

## Usage

Once the service is running, it will monitor CPU usage according to the rules defined. You can check the service's status at any time with:
```bash
sudo systemctl status jiankong
```
