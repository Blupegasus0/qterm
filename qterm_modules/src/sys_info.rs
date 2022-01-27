//------------------------System-Info----------------------------------------------------------------------------------------------------------------------

pub mod sys_info {
    use sysinfo::{ProcessorExt, SystemExt};

    pub fn sys_info(status: Option<bool>) -> ([String; 3], [String; 3]) {
        let mut system = sysinfo::System::new();

        // update all info for system struct
        system.refresh_all();

        // display cpu usage
        //// still not working quite right, percentages seem inaccurate
        let mut cpu_usage: f32 = 0.0;
        let mut count = 0.0;
        for processor in system.get_processor_list() {
            // displays individual cpu usage
            //println!("{}: {}", processor.get_name(), processor.get_cpu_usage());

            count = count + 1.0;
            cpu_usage = cpu_usage + processor.get_cpu_usage();
        }

        // calculates average core usage
        let total_cpu_usage = (cpu_usage / count * 100.0).round();
        let total_cpu_usage: u16 = total_cpu_usage as u16;

        // return average core usage as string arr
        let display_cpu = match status {
            Some(v) => {
                if v {
                    // displays if true
                    [
                        String::from("╔══════════╗"),
                        format!("║ CPU: {}% ║", total_cpu_usage),
                        String::from("╚══════════╝"),
                    ]
                } else {
                    // displays nothing otherwise
                    [String::from(""), String::from(""), String::from("")]
                }
            }
            // shows error msg
            _ => [
                String::from("Set show-sys-info = true/false in ~/.config/qterm/qterm.toml"),
                String::from(""),
                String::from(""),
            ],
        };

        // finally, RAM and swap
        let total_ram: f64 = system.get_total_memory() as f64;
        let used_ram: f64 = system.get_used_memory() as f64;
        let percent_ram: f64 = (&used_ram / &total_ram * 100.0).round();
        let percent_ram: u8 = percent_ram as u8;

        // returns ram as a string arr
        let display_ram = match status {
            Some(v) => {
                if v {
                    // displays if true
                    [
                        String::from("╔══════════╗"),
                        format!("║ RAM: {}% ║", percent_ram),
                        String::from("╚══════════╝"),
                    ]
                } else {
                    // displays nothing otherwise
                    [String::from(""), String::from(""), String::from("")]
                }
            }
            // shows error msg
            _ => [
                String::from("Set show-sys-info = true/false in ~/.config/qterm/qterm.toml"),
                String::from(""),
                String::from(""),
            ],
        };

        (display_cpu, display_ram)
    }
}
