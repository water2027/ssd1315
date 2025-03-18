pub enum IoEvent {
    None,
    Clear,
    Sleep,
    Wake,
    Write(usize, usize, String),
}

impl IoEvent {
    pub fn new(line: &String) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let len = parts.len();

        if len == 0 {
            println!("empty!");
            return IoEvent::None;
        }

        if len == 1 {
            match parts[0] {
                "clear" => {
                    return IoEvent::Clear;
                }
                "sleep" => {
                    return IoEvent::Sleep;
                }
                "wake" => {
                    return IoEvent::Wake;
                }
                &_ => {
                    return IoEvent::None;
                }
            }
        }

        if len >= 3 {
            if let (Ok(x), Ok(y)) = (parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                let text = parts[3..].join(" ");
                return IoEvent::Write(x, y, text);
            }
        }

        IoEvent::None
    }
}
