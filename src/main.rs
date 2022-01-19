//Terminal App to display some ascii art
//and display useful info like time/date/system resources
use chrono::Timelike;
use colored::Colorize;
use rand::Rng;
use std::path::Path;
use sysinfo::{ComponentExt, ProcessorExt, SystemExt};
use toml::Value;

fn main() {
    // config file path
    let path = Path::new("/home/amire/.config/qterm/qterm.toml");

    // read config file to string
    let config = std::fs::read_to_string(&path).expect("cannot read file");

    // sets value as the content of the config file
    // can now be parsed for key value pairs
    let value = config.parse::<Value>().unwrap();

    // sets the config variables as Option<&str>
    let show_art = value["show-ascii-art"].as_bool();
    //let show_welcome = value["show-welcome-text"].as_bool();
    let show_time = value["show-time"].as_bool();
    //let time_format =value["time-format"].as_str();
    let show_date = value["show-date"].as_bool();
    //let date_format = value["date-format"].as_str();
    let show_sys_info = value["show-sys-info"].as_bool();
    //let sys_info_format =
    let info_color = match value["info-color"].as_str() {
        Some(b) => b,
        _ => "none",
    };

    // terminal art
    match show_art {
        Some(v) => {
            if v {
                ascii_art();
            }
        }
        _ => println!("Set show-ascii-art = true/false in ~/.config/qterm/qterm.toml"),
    }

    // displays info horizontally
    let time_strs = time(show_time);
    //let date_strs = date(show_date);
    let cpu_strs = sys_info(show_sys_info).0;
    let ram_strs = sys_info(show_sys_info).1;

    for i in 0..3 {
        match info_color {
            // red, blue, green, yellow, cyan, magenta, black, grey,
            _ => {
                println!(
                    "{} {} {}",
                    time_strs[i].white(),
                    cpu_strs[i].blue(),
                    ram_strs[i].red()
                );
            }
        }
    }

    // SET TO CHANGE

    // date
    match show_date {
        Some(v) => {
            if v {
                date();
            }
        }
        _ => println!("Set show-date = true/false in ~/.config/qterm/qterm.toml"),
    }
} // main end

fn ascii_art() {
    // display art randomly
    let rand_art = rand::thread_rng().gen_range(1..11);
    let rand_color = rand::thread_rng().gen_range(1..10);

    // list of ascii art
    let art1 = "
     _    _      _ _        __          __        _     _ _ 
    | |  | |    | | |       \\ \\        / /       | |   | | |
    | |__| | ___| | | ___    \\ \\  /\\  / /__  _ __| | __| | |
    |  __  |/ _ \\ | |/ _ \\    \\ \\/  \\/ / _ \\| '__| |/ _` | |
    | |  | |  __/ | | (_) |    \\  /\\  / (_) | |  | | (_| |_|
    |_|  |_|\\___|_|_|\\___/      \\/  \\/ \\___/|_|  |_|\\__,_(_)
        "
    .bold();

    let art2 = "           _   _  ____  _   ___     ____  __  ____  _    _  _____ 
     /\\   | \\ | |/ __ \\| \\ | \\ \\   / /  \\/\\ |/ __ \\| |  | |/ ____|
    /  \\  |  \\| | |  | |  \\| |\\ \\_/ /| \\  / | |  | | |  | | (___  
   / /\\ \\ | . ` | |  | | . ` | \\   / | |\\/| | |  | | |  | |\\___ \\ 
  / ____ \\| |\\  | |__| | |\\  |  | |  | |  | | |__| | |__| |____) |
 /_/    \\_\\_| \\_|\\____/|_| \\_|  |_|  |_|  |_|\\____/ \\____/|_____/ 
                                                                  "
    .bold();
    let art3 = "

                              ██████░░░░░░░░░░░░░░░░██████                              
                            ██░░░░░░                ░░░░░░██                            
                          ██░░                            ░░██                          
                        ██░░                                ░░██                        
                        ██    ██████                ██████    ██                        
                        ██  ░░░░░░░░████        ████░░░░░░░░  ██                        
                        ██          ░░████    ████░░          ██                        
                        ██            ░░░░    ░░░░            ██                        
                        ██░░  ░░██████░░░░    ░░░░██████░░  ░░██                        
                        ██░░░░██████████░░    ░░██████████░░░░██                        
                        ██░░  ░░░░░░░░  ░░    ░░  ░░░░░░░░  ░░██                        
                        ██              ░░    ░░              ██                        
                        ██  ░░░░░░      ░░    ░░      ░░░░░░  ██                        
                        ██  ░░░░░░    ░░        ░░    ░░░░░░  ██                        
                        ██░░          ░░        ░░          ░░██                        
                        ██░░░░██        ██░░░░██        ██░░░░██                        
                        ██░░  ██████░░████████████░░██████  ░░██                        
                        ██  ░░  ██████████    ██████████  ░░  ██                        
                          ██  ░░░░    ░░░░░░░░░░░░    ░░░░  ██                          
                          ██      ░░                ░░      ██                          
                            ██  ░░  ░░░░░░████░░░░░░  ░░  ██                            
                            ██░░  ░░      ████      ░░  ░░██                            
                              ██░░      ░░████░░      ░░██                              
                                ██░░    ░░████░░    ░░██                                
                                  ██░░░░  ████  ░░░░██                                  
                                    ████░░████░░████                                    
                                        ████████                                        
                                                                                        
"
    .bold();

    // add syntax highlighting lmao
    let art4 = "
  _   __      __                         _                    _                       __      __
 (_) / _|    / /                        | |          ______  | |                      \\ \\    / /
  _ | |_    | |    __ _ __      __ __ _ | | __ ___  |______| | |_  _ __  _   _   ___   | |  | | 
 | ||  _|   | |   / _` |\\ \\ /\\ / // _` || |/ // _ \\  ______  | __|| '__|| | | | / _ \\  | | / /  
 | || |     | |  | (_| | \\ V  V /| (_| ||   <|  __/ |______| | |_ | |   | |_| ||  __/  | | \\ \\  
 |_||_|     | |   \\__,_|  \\_/\\_/  \\__,_||_|\\_\\___|            \\__||_|    \\__,_| \\___|  | |  | | 
           _ \\_\\                                __ __                                 /_/    \\_\\
          | |                                  / / \\ \\   _                                               
          | |__    __ _  _ __   _ __   _   _  | |   | | (_)                                              
          | '_ \\  / _` || '_ \\ | '_ \\ | | | | | |   | |                                                  
          | | | || (_| || |_) || |_) || |_| | | |   | |  _                                               
          |_| |_| \\__,_|| .__/ | .__/  \\__, | | |   | | ( )                                              
 __                     | |    | |      __/ |  \\_\\ /_/  |/                                               
 \\ \\                    |_|    |_|     |___/                                                             
  | |                                                                                           
   \\ \\                                                                                          
   / /                                                                                          
  | |                                                                                           
 /_/                                                                                            
                                                                                                "
.blue().bold();

    let art5 =
        "                                                                                        
                                                                                        
                                                                                        
                                                                                        
                                                                                        
                                                                                        
                                                                                        
                                            ████                                        
                                          ██    ██                                      
                                        ██  ▓▓▓▓  ██                                    
                                        ██  ▓▓▓▓  ██                                    
                                      ██            ██                                  
                                      ██    ▓▓▓▓    ██                                  
                                      ██    ▓▓▓▓    ██                                  
                                      ██            ██                                  
                                        ██  ▓▓▓▓  ██                                    
                                        ██  ▓▓▓▓  ██                                    
                                      ██▒▒██    ██▒▒██                                  
                                      ██▒▒██    ██▒▒██                                  
                                      ██▒▒████████▒▒██                                  
                                      ██▒▒██    ██▒▒██                                  
                                        ██        ██                                    
                                                                                        
                                                                                        
                                                                                        
                                                                                        
                                                                                        
                                                                                        
░░                                          ░░  ░░                                      
"
        .bold();

    let art6 =
        "                                                                                        
                                                                                        
            ░░                                                                  ░░      
                                                                                        
                                                                                        
      ░░                                ░░  ██                                          
  ░░                                        ██                              ░░░░        
                                            ██                                          
                                          ▓▓▓▓▓▓                                        
                                        ▓▓▓▓▓▓▓▓▓▓                                      
                                      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓                                    
                                      ▓▓▒▒▓▓▓▓▒▒▓▓▓▓                                    
                                      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓                                    
                                      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓                                    
                                      ░░░░▒▒░░░░░░░░                                    
                                      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓                                    
                                      ▓▓▒▒▒▒▓▓▒▒▒▒▓▓                                    
                                      ▓▓▒▒▒▒██▒▒▒▒▓▓                                    
                                    ████▓▓▓▓▓▓██▓▓▓▓██      ░░                          
                                  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██                                
                                ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██                              
                                ▓▓▓▓▓▓██▓▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓                              
        ░░                      ▓▓▓▓▓▓░░████████████░░████                        ░░    
                                ▓▓    ▓▓░░░░░░▓▓░░░░    ▓▓                              
                                                                                        
                                                                                        
                                                                                        
                                                                                        
                        ▒▒                ▒▒    ▒▒          ▒▒    ▒▒                    
"
        .bold();

    let art7 = "                _                       
                \\`*-.                   
                 )  _`-.                
                .  : `. .               
                : _   '  \\              
                ; *` _.   `*-._         
                `-.-'          `-.      
                  ;       `       `.    
                  :.       .        \\   
                  . \\  .   :   .-'   .  
                  '  `+.;  ;  '      :  
                  :  '  |    ;       ;-.
                  ; '   : :`-:     _.`* ;
         [bug] .*' /  .*' ; .*`- +'  `*'
               `*-*   `*-*  `*-*'       
"
    .bold();

    let art8 = "     _      _      _      _      _      _      _
   _( )__ _( )__ _( )__ _( )__ _( )__ _( )__ _( )__
 _|     _|     _|     _|     _|     _|     _|     _|
(_   _ (_   _ (_   _ (_   _ (_   _ (_   _ (_   _ (_
 |__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|
 |_     |_     |_     |_     |_     |_     |_     |_
  _) _   _) _   _) _   _) _   _) _   _) _   _) _   _)
 |__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|
 _|     _|     _|     _|     _|     _|     _|     _|
(_   _ (_   _ (_   _ (_   _ (_   _ (_   _ (_   _ (_
 |__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|
 |_     |_     |_     |_     |_     |_     |_     |_
  _) _   _) _   _) _   _) _   _) _   _) _   _) _mx _)
 |__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|__( )_|
"
    .bold();

    let art9 = "Art by Horroroso
.    ,'    ,--'   ,'    `;-.     / \\      `. o  ,--.      `.   /
 `.-'    ,'      '`-.,  /       /   `.      `--'  o `.      \\ /     `-
   `.   /       /  '-..,       ;    ,-`.          ,---`.,.---'
     \\ /     `-;.    ,'    ,--'   ,'    `;-.     / \\      `. o  ,--.
'`.---'          `.-'    ,'      '`-.,  /       /   `.      `--'  o `.
   `. o  ,--.      `.   /       /  '-..,       ;    ,-`.          ,---
     `--'  o `.      \\ /     `-;.    ,'    ,--'   ,'    `;-.     / \\
.          ,---`'`.---'          `.-'    ,'      '`-.,  /       /   `.
 `;-.     / \\      `. o  ,--.      `.   /       /  '-..,       ;    ,-
 /       /   `.      `--'  o `.      \\ /     `-;.    ,'    ,--'   ,'
,       ;    ,-`.          ,---`.,.---'          `.-'    ,'      '`-.,
    ,--'   ,'    `;-.     / \\      `. o  ,--.      `.   /       /  '-.
  ,'      '`-.,  /       /   `.      `--'  o `.      \\ /     `-;.    ,
 /       /  '-..,       ;    ,-`.          ,---`'`.---'          `.-'
/     `-;.    ,'    ,--'   ,'    `;-.     / \\      `. o  ,--.      `.
          `.-'    ,'      '`-.,  /       /   `.      `--'  o `.      \\
  ,--.      `.   /       /  '-..,       ;    ,-`.          ,---`'`.---
-'  o `.      \\ /     `-;.    ,'    ,--'   ,'    `;-.     / \\      `.
    ,---`'`.---'          `.-'    ,'      '`-.,  /       /   `.      `
   / \\      `. o  ,--.      `.   /       /  '-..,       ;    ,-`.
  /   `.      `--'  o `.      \\ /     `-;.    ,'    ,--'   ,'    `;-.
 ;    ,-`.          ,---`'`.---'          `.-'    ,'      '`-.,  /
'   ,'    `;-.     / \\      `. o  ,--.      `.   / -hrr- /  '-..,
   '`-.,  /       /   `.      `--'  o `.      \\ /     `-;.    ,'    ,-"
        .bold();

    let art10 = "that".bold();

    // art to be displayed
    let display_art = match rand_art {
        1 => art1,
        2 => art2,
        3 => art3,
        4 => art4,
        5 => art5,
        6 => art6,
        7 => art7,
        8 => art8,
        9 => art9,
        10 => art10,
        _ => art1,
    };

    // the final value is printed
    match rand_color {
        1 => println!("{}\n", display_art.red()),
        2 => println!("{}\n", display_art.blue()),
        3 => println!("{}\n", display_art.green()),
        4 => println!("{}\n", display_art.yellow()),
        5 => println!("{}\n", display_art.purple()),
        6 => println!("{}\n", display_art.white()),
        7 => println!("{}\n", display_art.bright_black()),
        8 => println!("{}\n", display_art.bright_red()),
        9 => println!("{}\n", display_art.bright_cyan()),
        _ => println!("{}\n", display_art.blue()),
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------------------

fn time(status: Option<bool>) -> [String; 3] {
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

    /*let display_time = format!(
            "╔══════════╗
    ║ 🕐 {}:{} ║
    ╚══════════╝",
            hour, minute
        )
        .bold(); // allow display color to be determined by config file */

    // displays the time
    //println!("{}", display_time);

    display_time
}

//-------------------------------------------------------------------------------------------------------------------------------------------------

// more like day: displays the day of the week in large ascii letters!
fn date() {}

//-------------------------------------------------------------------------------------------------------------------------------------------------

fn sys_info(status: Option<bool>) -> ([String; 3], [String; 3]) {
    let mut system = sysinfo::System::new();

    // update all info for system struct
    system.refresh_all();

    // print id and name for every process
    //for (pid, proc_) in system.get_process_list() {
    //println!("{} => status: {:?}", pid, proc_);
    //}

    // print the temp of the different components
    //for component in system.get_components_list() {
    //println!("{}: {}", component.get_label(), component.get_temperature());
    //}

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
                // displays time if true
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

    // and then the disk info
    //for disk in system.get_disks() {
    //println!("{:?}", disk)
    //}

    // finally, RAM and swap
    let total_ram: f64 = system.get_total_memory() as f64;
    let used_ram: f64 = system.get_used_memory() as f64;
    let percent_ram: f64 = (&used_ram / &total_ram * 100.0).round();
    let percent_ram: u8 = percent_ram as u8;

    // displays ram sizes in kilobytes
    //println!("total memory: {}kb", &total_ram);
    //println!("used memory: {}kb", &used_ram);

    // return average core usage as string arr
    let display_ram = match status {
        Some(v) => {
            if v {
                // displays time if true
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
