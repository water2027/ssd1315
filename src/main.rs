mod font8x8;
mod ssd1315;

use ssd1315::SSD1315;

const SSD1315_ADDR: u16 = 0x3c;
fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    ins.draw_text(0, 0, "hello,world!");
    ins.display()?;

    ins.set_dim()?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    ins.set_contrast(0xcf)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    ins.sleep()?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    ins.wake()?;

    Ok(())
}
