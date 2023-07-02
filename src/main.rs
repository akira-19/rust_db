// use meta::btree::*;

// mod meta;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut server_mode = false;

    for arg in args.iter() {
        if arg == "-server" {
            server_mode = true;
        }
    }

    // ここからプログラムの処理を実行
    // ...

    if server_mode {
        // サーバーモードの処理
        // ...
    } else {
        // サーバーモードでない場合の処理
        // ...
    }
}
