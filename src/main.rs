use enigo::{Enigo, MouseButton, MouseControllable};
use mouse_position::mouse_position::Mouse;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::{thread::sleep, time::Duration};

#[derive(Deserialize, Serialize)]
struct Config {
    fps: u64,
    upper_left: Position,
    lower_right: Position,
}

#[derive(Deserialize, Serialize)]
struct Position {
    x: i32,
    y: i32,
}

const A_SECOND: u64 = 1_000_000_000;

fn main() {
    let config: Config = if let Ok(toml) = fs::read_to_string("config.toml") {
        if let Ok(c) = toml::from_str(&toml) {
            c
        } else {
            eprintln!("config.tomlが読み込めません。");
            return;
        }
    } else {
        match init_config() {
            Ok(c) => c,
            Err(()) => {
                return;
            }
        }
    };

    let frame = A_SECOND / config.fps;
    println!("1f is {frame} nanosec");

    readline("Enterキーを押すと連射を開始します。");

    let mut enigo = Enigo::new();
    loop {
        let pos = Mouse::get_mouse_position();
        let (x, y) = match pos {
            Mouse::Position { x, y } => (x, y),
            Mouse::Error => {
                eprintln!("マウス位置の取得に失敗しました。");
                readline("");
                break;
            }
        };
        if config.upper_left.x < x
            && x < config.lower_right.x
            && config.upper_left.y < y
            && y < config.lower_right.y
        {
            enigo.mouse_click(MouseButton::Left);
            sleep(Duration::from_nanos(frame));
        }
    }
}

fn init_config() -> Result<Config, ()> {
    loop {
        println!("設定ファイルが見つかりません。初期設定を実行します。");
        let Ok(fps): Result<u64, _> = readline("1秒間に何回クリックしますか？").parse()
        else {
            println!("* 正の整数を入力してください。");
            continue;
        };
        println!("連射を有効にする範囲を記録します。");
        readline("連射を有効にしたい範囲の左上にカーソルを合わせ、Enterを押してください。");
        let pos = Mouse::get_mouse_position();
        let (min_x, min_y) = match pos {
            Mouse::Position { x, y } => (x, y),
            Mouse::Error => {
                eprintln!("マウス位置の取得に失敗しました。");
                readline("");
                return Err(());
            }
        };
        readline("連射を有効にしたい範囲の右下にカーソルを合わせ、Enterを押してください。");
        let pos = Mouse::get_mouse_position();
        let (max_x, max_y) = match pos {
            Mouse::Position { x, y } => (x, y),
            Mouse::Error => {
                eprintln!("マウス位置の取得に失敗しました。");
                readline("");
                return Err(());
            }
        };

        let config = Config {
            fps,
            upper_left: Position { x: min_x, y: min_y },
            lower_right: Position { x: max_x, y: max_y },
        };

        let toml = toml::to_string(&config).expect("Unexpected Error.");
        let mut file = fs::File::create("config.toml").expect("IO Error.");
        file.write_all(toml.as_bytes()).expect("IO Error.");
        println!("完了しました。");
        return Ok(config);
    }
}

fn readline(prompt: &str) -> String {
    print!("{prompt}: ");
    stdout().flush().expect("IO Error.");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("IO Error.");
    buffer.trim().to_owned()
}
