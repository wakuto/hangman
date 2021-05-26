use std::collections::HashMap;
use std::io::{stdout, Write};
use std::fs;
use std::env;
use rand::{Rng, thread_rng};

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name: &str = match args.len() {
    2 => &args[1],
    _ => "/home/wakuto/src/2021-prog_enshuu/hangman/words",
  };

  let mut collect_count = 0;
  let mut play_count = 0;

  let words_file = fs::read_to_string(file_name).expect("failed to read");
  let words_vec: Vec<&str> = words_file.split('\n').collect();  // ワード一覧
  let mut rng = thread_rng(); // 乱数発生源

  loop {
    play_count += 1;
    let mut turn = 10;    // 残りのターン
    let mut input_char = HashMap::new();  // 入力した文字
    let target = words_vec[rng.gen_range(0..=words_vec.len())];   // ターゲット
    
    // ゲームのメインループ
    while turn > 0 {
      // print process
      println!("");
      let collect_flag = print_and_check(target, &input_char);
      if collect_flag {
        println!("Collect!!");
        collect_count += 1;
        break;
      }
      println!("残り回数：{}", turn);
    
      // input process
      print!("please input>>");
      stdout().flush().unwrap();
      let mut ch = String::new();
      std::io::stdin().read_line(&mut ch).expect("failed to read.");
      ch = ch.trim().to_string();

      if ch.len() < 1 {
        continue;
      }

      // check if input has entered
      let ch = ch.chars().next().expect("failed to to_char").clone();
      if input_char.contains_key(&ch) {
        input_char.insert(ch.clone(), input_char.get(&ch).unwrap()+1);
      } else {
        input_char.insert(ch.clone(), 1);
      }

      if input_char.get(&ch).unwrap().clone() != 1 {
        continue;
      }

      // check if input has collect
      if !target.contains(&ch.to_string()) {
        turn -= 1;
      }

      // 残り回数0のときの終了処理
      if turn == 0 {
        println!("You lose...");
        println!("The answer is {}.", target);
        break;
      }
    }

    let continue_flag;
    let mut yesorno = String::new();

    // 続けますかループ
    loop {
      print!("続けますか？y/n>");
      stdout().flush().unwrap();
      std::io::stdin().read_line(&mut yesorno).expect("failed to read.");
      yesorno = yesorno.trim().to_string();

      // yならつづける、それ以外なら終わる。 １文字じゃない場合はもう一度読み取り
      if yesorno.len() == 1 {
        continue_flag = match yesorno.chars().nth(0).unwrap() {
          'y' => true,
          _ => false,
        };
        break;
      }
    }
    if !continue_flag {
      println!("your score is {}/{}/{}", collect_count, play_count-collect_count, (collect_count as f32)/(play_count as f32));
      break;
    }
  }
}

fn print_and_check(target: &str, input_char: &HashMap<char, u32>) -> bool {
  let mut chars = target.chars();
  let mut collect_flag = true;
  print!("使われた文字：");
  for ch in input_char.keys() {
    print!("{}", ch);
  }
  print!("\n単語：");
  for _i in 0..target.len() {
    let next = chars.next().unwrap().clone();
    if input_char.contains_key(&next) {
      print!("{}", next);
    } else {
      print!("_");
      collect_flag = false;
    }
  }
  println!("");
  collect_flag
}
