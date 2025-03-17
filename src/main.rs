mod font8x8;
mod io_handler;
mod message;
mod ssd1315;
use crate::io_handler::IoHandler;
use crate::message::IoEvent;
use ssd1315::SSD1315;

use std::sync::mpsc::{channel, Receiver, Sender};

const SSD1315_ADDR: u16 = 0x3c;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx): (Sender<IoEvent>, Receiver<IoEvent>) = channel();

    let handler = IoHandler::new(tx);

    std::thread::spawn(move || {
        let mut ins = SSD1315::new(SSD1315_ADDR);

        if ins.init().is_err() {
            println!("\n============ 硬件连接诊断 ============\n");
            println!("1. 确认I2C接口已启用 (sudo raspi-config -> Interface Options -> I2C)\n");
            println!("2. 验证连接正确:\n");
            println!("   VCC -> 3.3V 或 5V (取决于模块需求)\n");
            println!("   GND -> GND\n");
            println!("   SCL -> GPIO 3 (物理引脚5)\n");
            println!("   SDA -> GPIO 2 (物理引脚3)\n");
            println!("3. 验证I2C总线:\n");
            println!("   运行 'sudo i2cdetect -y 1' 查看连接的设备\n");
            println!("4. 如果地址不是0x3C，请修改代码中的SSD1315_ADDR\n");
            println!("=====================================\n");
            panic!("fail to connect");
        }

        println!("SSD1315 初始化完成");

        while let Ok(cmd) = rx.recv() {
            match cmd {
                IoEvent::Wake => {
                    if ins.wake().is_err() {
                        println!("wake error");
                    }
                }
                IoEvent::Clear => {
                    ins.clear();
                    if ins.display().is_err() {
                        println!("display error");
                    }
                }
                IoEvent::Sleep => {
                    if ins.sleep().is_err() {
                        println!("sleep error");
                    }
                }
                IoEvent::Write(x, y, text) => {
                    ins.draw_text(x, y, text.as_str());
                    if ins.display().is_err() {
                        println!("display error");
                    }
                }
                _ => {
                    println!("none");
                }
            }
        }
    });

    handler.run();

    Ok(())
}
