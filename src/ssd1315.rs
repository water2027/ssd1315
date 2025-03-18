use crate::font8x8;
use rppal::i2c::{Error, I2c};

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

pub struct SSD1315 {
    ins: I2c,
    buffer: [u8; SSD1315_BUFFER_SIZE],
}

impl SSD1315 {
    pub fn new(addr: u16) -> Self {
        let mut ins = I2c::new().unwrap();
        ins.set_slave_address(addr).unwrap();
        let buffer: [u8; SSD1315_BUFFER_SIZE] = [0; SSD1315_BUFFER_SIZE];
        SSD1315 { ins, buffer }
    }

    pub fn init(&mut self) -> Result<(), Error> {
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

    pub fn reset(&mut self) -> Result<(), Error> {
        println!("软件重置");

        self.send_command(SSD1315_SOFT_RESET)?;

        Ok(())
    }

    /// 清除缓冲区buffer
    pub fn clear(&mut self) {
        println!("清空缓存区");
        self.buffer.fill(0);
    }

    /// Set the display to a dimmed state (lower contrast)
    /// level should be between 0 (minimum contrast) and 255 (maximum contrast)
    pub fn set_contrast(&mut self, level: u8) -> Result<(), Error> {
        println!("设置对比度: {}", level);
        self.send_command(SSD1315_SET_CONTRAST)?;
        self.send_command(level)?;
        Ok(())
    }

    /// Put the display into dim mode (very low contrast to save power)
    pub fn set_dim(&mut self) -> Result<(), Error> {
        println!("设置低亮度模式");
        self.set_contrast(0x0F) // Very low contrast value
    }

    /// Put the display into sleep mode (lowest power consumption)
    /// Note: Display RAM content is preserved, but display is off
    pub fn sleep(&mut self) -> Result<(), Error> {
        println!("进入睡眠模式");
        // Turn off the display
        self.send_command(SSD1315_DISPLAY_OFF)?;

        // Disable the charge pump
        self.send_command(SSD1315_CHARGE_PUMP)?;
        self.send_command(0x10)?; // Disable charge pump

        Ok(())
    }

    /// Wake up the display from sleep mode
    pub fn wake(&mut self) -> Result<(), Error> {
        println!("退出睡眠模式");

        // Re-enable the charge pump
        self.send_command(SSD1315_CHARGE_PUMP)?;
        self.send_command(0x14)?; // Enable charge pump

        // Turn on the display
        self.send_command(SSD1315_DISPLAY_ON)?;

        // You might need to refresh the display content here
        self.display()?;

        Ok(())
    }

    /// 将缓冲区内容发送到显示器
    /// 如果未开启显示器，会开启显示器
    pub fn display(&mut self) -> Result<(), Error> {
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

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
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
            ',' => 36,   // Comma
            '.' => 37,   // Period
            '!' => 38,   // Exclamation mark
            '?' => 39,   // Question mark
            _ => return, // Unsupported character, don't draw
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

    /// 绘制文本
    /// x, y: 文本左上角的坐标
    /// text: 要显示的文本
    pub fn draw_text(&mut self, x: usize, y: usize, text: &str) {
        let mut current_x = x;
        let mut current_y = y;

        for c in text.chars() {
            // 处理换行符
            if c == '\n' {
                current_x = x; // 回到初始x位置
                current_y += 1; // 移动到下一行
                continue;
            }

            // 检查是否需要自动换行
            if (current_x + 1) * 8 > SSD1315_WIDTH as usize {
                current_x = x;
                current_y += 1; // 移动到下一行
            }

            // 检查是否超出屏幕底部
            if (current_y + 1) * 8 > SSD1315_HEIGHT as usize {
                break;
            }

            // 绘制字符
            self.draw_char(current_x * 8, current_y * 8, c);

            // 移动到下一个字符位置
            current_x += 1; // 每个字符宽度为8像素
        }
    }
    pub fn draw_processed_bitmap(&mut self, x: usize, y: usize, bitmap: &[u8], w: usize, h: usize) {
        if bitmap.len() < w * ((h + 7) / 8) {
            return; // 数组太小，无法包含指定尺寸的位图
        }

        let pages = (h + 7) / 8;

        for page in 0..pages {
            if y + page * 8 >= SSD1315_HEIGHT as usize {
                break;
            }

            for x_pos in 0..w {
                if x + x_pos >= SSD1315_WIDTH as usize {
                    break;
                }

                let src_index = x_pos + page * w;
                if src_index < bitmap.len() {
                    let byte_value = bitmap[src_index];

                    // 计算目标缓冲区中对应的页和索引
                    let dst_page = (y / 8) + page;
                    let y_offset = y % 8;
                    let dst_index = (x + x_pos) + dst_page * (SSD1315_WIDTH as usize);

                    if dst_index < self.buffer.len() {
                        if y_offset == 0 {
                            // 页对齐，可以直接写入
                            self.buffer[dst_index] = byte_value;
                        } else {
                            // 需要分两部分写入
                            // 先清除目标位置的相应位
                            let mask_lower = !(0xFF << y_offset);
                            let mask_upper = !(0xFF >> (8 - y_offset));

                            self.buffer[dst_index] &= mask_lower;
                            self.buffer[dst_index] |= byte_value << y_offset;

                            // 处理跨页的部分
                            if dst_page + 1 < (SSD1315_HEIGHT as usize) / 8 {
                                let next_index = dst_index + (SSD1315_WIDTH as usize);
                                if next_index < self.buffer.len() {
                                    self.buffer[next_index] &= mask_upper;
                                    self.buffer[next_index] |= byte_value >> (8 - y_offset);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
