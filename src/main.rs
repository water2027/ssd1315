use rppal::i2c::{Error, I2c};

mod font8x8;

const SSD1315_ADDR: u16 = 0x3c;
const SSD1315_WIDTH: u8 = 128;
const SSD1315_HEIGHT: u8 = 64;
const SSD1315_BUFFER_SIZE: usize = (SSD1315_WIDTH as usize) * (SSD1315_HEIGHT as usize) / 8;

const SSD1315_COMMAND: u8 = 0x00; // 命令控制
const SSD1315_DATA: u8 = 0x40; // 数据控制

// 显示命令
const SSD1315_DISPLAY_OFF: u8 = 0xae; // 关闭显示
const SSD1315_DISPLAY_ON: u8 = 0xaf; // 开启显示
const SSD1315_SET_CONTRAST: u8 = 0x81; // 设置对比度
const SSD1315_NORMAL_DISPLAY: u8 = 0xa6; // 正常显示(1=亮)
const SSD1315_INVERT_DISPLAY: u8 = 0xa7; // 反转显示(0=亮)

// 滚动命令
const SSD1315_DEACTIVEATE_SCROLL: u8 = 0x2e; // stop scroll
const SSD1315_ACTIVATE_SCROLL: u8 = 0x2f; // start scroll

// 寻址命令
const SSD1315_SET_MEMORY_ADDR_MODE: u8 = 0x20; // 设置内存寻址模式
const SSD1315_SET_COLUMN_ADDR: u8 = 0x21; // 设置列地址范围
const SSD1315_SET_PAGE_ADDR: u8 = 0x22; // 设置页地址范围
const SSD1315_SET_START_LINE: u8 = 0x40; // 设置显示起始行

// 硬件配置命令
const SSD1315_SET_DISPLAY_OFFSET: u8 = 0xd3;
const SSD1315_SET_SEGMENT_REMAP: u8 = 0xa0;
const SSD1315_SET_COM_SCAN_DIR: u8 = 0xc0;
const SSD1315_SET_COM_PINS: u8 = 0xda;
const SSD1315_SET_MULTIPLEX_RATIO: u8 = 0xa8;

// 时序和驱动命令
const SSD1315_SET_DISPLAY_CLOCK: u8 = 0xd5; // 设置显示时钟
const SSD1315_SET_PRECHARGE: u8 = 0xd9; // 设置预充电周期
const SSD1315_SET_VCOM_DETECT: u8 = 0xd8; // 设置VCOMH电压
const SSD1315_CHARGE_PUMP: u8 = 0x8d; // 设置电荷泵

// 软件重置
const SSD1315_SOFT_RESET: u8 = 0xe3;

struct SSD1315 {
    ins: I2c,
    buffer: [u8; SSD1315_BUFFER_SIZE],
}

impl SSD1315 {
    fn new(addr: u16) -> Self {
        let mut ins = I2c::new().unwrap();
        ins.set_slave_address(addr).unwrap();
        let buffer: [u8; SSD1315_BUFFER_SIZE] = [0; SSD1315_BUFFER_SIZE];
        SSD1315 { ins, buffer }
    }

    fn init(&mut self) -> Result<(), Error> {
        self.reset()?;

        println!("开始配置显示屏");

        self.send_command(SSD1315_DISPLAY_OFF)?;

        self.send_command(SSD1315_SET_DISPLAY_CLOCK)?;
        self.send_command(0x80)?;

        self.send_command(SSD1315_SET_MULTIPLEX_RATIO)?;
        self.send_command(0x3f)?;

        self.send_command(SSD1315_SET_DISPLAY_OFFSET)?;
        self.send_command(0x00)?;

        self.send_command(SSD1315_SET_START_LINE | 0x00)?;

        self.send_command(SSD1315_CHARGE_PUMP)?;
        self.send_command(0x14)?;

        self.send_command(SSD1315_SET_MEMORY_ADDR_MODE)?;
        self.send_command(0x00)?;

        self.send_command(SSD1315_SET_SEGMENT_REMAP | 0x01)?;
        self.send_command(SSD1315_SET_COM_SCAN_DIR | 0x08)?;

        self.send_command(SSD1315_SET_COM_PINS)?;
        self.send_command(0x12)?;

        self.send_command(SSD1315_SET_CONTRAST)?;
        self.send_command(0xcf)?;

        self.send_command(SSD1315_SET_PRECHARGE)?;
        self.send_command(0xf1)?;

        self.send_command(SSD1315_SET_VCOM_DETECT)?;
        self.send_command(0x40)?;

        self.send_command(SSD1315_DEACTIVEATE_SCROLL)?;

        self.send_command(SSD1315_NORMAL_DISPLAY)?;

        self.clear();
        self.display()?;

        Ok(())
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), Error> {
        self.ins.write(&[SSD1315_COMMAND, cmd])?;
        Ok(())
    }

    fn send_data(&mut self, data: u8) -> Result<(), Error> {
        self.ins.write(&[SSD1315_DATA, data])?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        println!("软件重置");

        self.send_command(SSD1315_SOFT_RESET)?;

        Ok(())
    }

    /// 清除缓冲区buffer
    fn clear(&mut self) {
        println!("清空缓存区");
        self.buffer.fill(0);
    }

    /// 将缓冲区内容发送到显示器
    /// 如果未开启显示器，会开启显示器
    fn display(&mut self) -> Result<(), Error> {
        println!("更新显示器内容");

        self.send_command(SSD1315_DISPLAY_ON)?;

        self.send_command(SSD1315_SET_COLUMN_ADDR)?;
        self.send_command(0)?;
        self.send_command(SSD1315_WIDTH - 1)?;

        self.send_command(SSD1315_SET_PAGE_ADDR)?;
        self.send_command(0)?;
        self.send_command(7)?;

        let mut success = 0;

        for data in self.buffer {
            if self.send_data(data).is_ok() {
                success += 1;
            }
        }

        println!(
            "成功发送{}/{} 字节数据到显示器\n",
            success, SSD1315_BUFFER_SIZE
        );

        Ok(())
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: u8) {
        if x >= (SSD1315_WIDTH as usize) || y >= (SSD1315_HEIGHT as usize) {
            return;
        }

        let page = y / 8;
        let bit = y % 8;
        let index = x + page * (SSD1315_WIDTH as usize);

        if color == 1 {
            self.buffer[index] |= 1 << bit;
        } else {
            self.buffer[index] &= !(1 << bit);
        }
    }

    fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };

        let dy = -(y1 - y).abs(); // 注意这里是负的
        let sy = if y < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.draw_pixel(x as usize, y as usize, 1);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                if x == x1 {
                    break;
                }
                err += dy;
                x += sx;
            }

            if e2 <= dx {
                if y == y1 {
                    break;
                }
                err += dx;
                y += sy;
            }
        }
    }

    /// 绘制单个字符 (8x8像素)
    /// x, y: 字符左上角的坐标
    /// c: 要显示的字符
    fn draw_char(&mut self, x: usize, y: usize, c: char) {
        // 只支持0-9, a-z
        let font_index = match c {
            '0'..='9' => c as usize - '0' as usize,
            'a'..='z' => c as usize - 'a' as usize + 10,
            _ => return, // 不支持的字符，不绘制
        };

        // 检查索引是否在范围内
        if font_index >= font8x8::FONT8X8.len() {
            return;
        }

        // 检查是否超出显示范围
        if x + 8 > SSD1315_WIDTH as usize || y + 8 > SSD1315_HEIGHT as usize {
            return;
        }

        let char_bitmap = font8x8::FONT8X8[font_index];

        // 绘制8x8字符
        for row in 0..8 {
            let row_data = char_bitmap[row];
            for col in 0..8 {
                let bit = (row_data >> (7 - col)) & 0x01;
                if bit > 0 {
                    self.draw_pixel(x + col, y + row, 1);
                }
            }
        }
    }
}

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


    ins.display()?;

    Ok(())
}
