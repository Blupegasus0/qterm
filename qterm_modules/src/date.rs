//------------------------Date-----------------------------------------------------------------------------------------------------------------------
pub mod date {
    use chrono::Timelike;

    pub fn time(status: Option<bool>) -> [String; 3] {
        let time = chrono::offset::Local::now().time();
        let hour: String = match time.hour() {
            0 => "00".to_string(),
            1 => "01".to_string(),
            2 => "02".to_string(),
            3 => "03".to_string(),
            4 => "04".to_string(),
            5 => "05".to_string(),
            6 => "06".to_string(),
            7 => "07".to_string(),
            8 => "08".to_string(),
            9 => "09".to_string(),
            _ => time.hour().to_string(),
        };
        let minute: String = match time.minute() {
            0 => "00".to_string(),
            1 => "01".to_string(),
            2 => "02".to_string(),
            3 => "03".to_string(),
            4 => "04".to_string(),
            5 => "05".to_string(),
            6 => "06".to_string(),
            7 => "07".to_string(),
            8 => "08".to_string(),
            9 => "09".to_string(),
            _ => time.minute().to_string(),
        };

        let display_time = match status {
            Some(v) => {
                if v {
                    // displays time if true
                    [
                        String::from("╔══════════╗"),
                        format!("║ 🕐 {}:{} ║", hour, minute),
                        String::from("╚══════════╝"),
                    ]
                } else {
                    // displays nothing otherwise
                    [String::from(""), String::from(""), String::from("")]
                }
            }
            // shows error msg
            _ => [
                String::from("Set show-time = true/false in ~/.config/qterm/qterm.toml"),
                String::from(""),
                String::from(""),
            ],
        };

        display_time
    }

    // more like day: displays the day of the week in large ascii letters!
    pub fn date() {}
}
