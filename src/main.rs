use env_logger::Env;
use log::info;
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::{thread, time::Duration};
use sysinfo::System;
use time::Error;
use time::{format_description, macros::offset, OffsetDateTime};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

#[derive(Deserialize)]
struct Config {
    email: EmailConfig,
}

#[derive(Deserialize)]
struct EmailConfig {
    from: String,
    to: String,
    smtp_username: String,
    smtp_password: String,
    smtp_relay: String,
}

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

    let mut sys = System::new_all();
    let mut shutdown_pending = false;
    let mut shutdown_start_time: Option<std::time::Instant> = None;
    let mut low_cpu_accum = 0u64; // 低于80%累计秒数
    let mut last_mail_time: Option<std::time::Instant> = None;
    loop {
        let cpu_usage = cpu(&mut sys, &mut last_mail_time, &config.email);
        if shutdown_pending {
            if cpu_usage < 80.0 {
                low_cpu_accum += 5; // 每次循环5秒
            } else {
                // 只要高于80%，不累计
            }
            if low_cpu_accum >= 60 {
                println!("5分钟内低负载累计超1分钟，取消关机");
                shutdown_pending = false;
                shutdown_start_time = None;
                low_cpu_accum = 0;
            } else if let Some(start) = shutdown_start_time {
                if start.elapsed().as_secs() >= 300 {
                    println!("5分钟到，执行关机");
                    match shutdown() {
                        Ok(_) => println!("ok"),
                        Err(e) => eprintln!("okk{}", e),
                    }
                    shutdown_pending = false;
                    shutdown_start_time = None;
                    low_cpu_accum = 0;
                }
            }
        } else if cpu_usage > 90.0 {
            println!("CPU占用率超过90%，5分钟后关机倒计时开始");
            shutdown_pending = true;
            shutdown_start_time = Some(std::time::Instant::now());
            low_cpu_accum = 0;
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn cpu(sys: &mut System, last_mail_time: &mut Option<std::time::Instant>, email_config: &EmailConfig) -> f32 {
    // 初始化系统信息
    thread::sleep(Duration::from_secs(2));
    // 刷新系统信息
    sys.refresh_all();
    thread::sleep(Duration::from_secs(5));
    // 获取所有进程
    let mut processes: Vec<_> = sys.processes().values().collect();

    // 按 CPU 使用率排序（从高到低）
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

    // 取前 3 个进程
    let top_processes = processes.iter().take(10);

    //根据cpu占用率，来执行任务
    let global_cpu = sys.global_cpu_usage();
    if global_cpu > 40.0 {
        //如果已经开始倒计时，则这里不显示下面的话，而是提示在倒计时中。
        if global_cpu>90.0{
            info!("{:?},cpu占用率为{}%", nowtime(),global_cpu);
        }else {
            info!("{:?},cpu占用率超过40%，开始记录", nowtime());
        }

        for (i, process) in top_processes.enumerate() {
            let process_info = format!(
                "{},{}(PID:{})-CPU Usage:{:.2}%",
                i + 1,
                process.name().to_string_lossy(),
                process.pid(),
                process.cpu_usage()
            );
            info!("{}", process_info);
        }
        if global_cpu > 80.0 {
            let now = std::time::Instant::now();
            let need_send = match last_mail_time {
                Some(t) => now.duration_since(*t).as_secs() > 3600, // 1小时发一次
                None => true,
            };
            if need_send {
                println!("发送邮件");
                send_email(
                    "警告：CPU占用率过高",
                    &format!("当前CPU占用率为{:.2}%，请注意服务器负载！", global_cpu),
                    email_config,
                );
                *last_mail_time = Some(now);
            }
        }
    } else {
        info!("一切顺利");
    }
    global_cpu
}

fn shutdown() -> Result<(), std::io::Error> {
    let output = Command::new("systemctl")
        .arg("poweroff") // 关机
        .spawn()?
        .wait()?;

    match output.success() {
        true => Ok(()),
        false => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Shutdown command failed",
        )),
    }
}

fn nowtime() -> Result<String, Error> {
    let now_utc = OffsetDateTime::now_utc();
    let now_beijing: OffsetDateTime = now_utc.to_offset(offset!(+8));

    // 自定义格式化字符串
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;

    // 使用自定义格式化字符串
    Ok(now_beijing.format(&format)?)
}

fn send_email(subject: &str, body: &str, config: &EmailConfig) {
    // 邮箱配置
    let email = Message::builder()
        .from(config.from.parse().unwrap())
        .to(config.to.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    // SMTP服务器配置
<<<<<<< HEAD
    let creds = Credentials::new("1637673094@qq.com".to_string(), "abodspoiotqacddg".to_string());
=======
    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
>>>>>>> cd06740 (Refactor: Move sensitive email information to config.toml and update main.rs to read from it.)

    // 这里以QQ邮箱为例，其他邮箱请更换smtp服务器
    let mailer = SmtpTransport::relay(&config.smtp_relay)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("邮件发送成功"),
        Err(e) => println!("邮件发送失败: {:?}", e),
    }
}
