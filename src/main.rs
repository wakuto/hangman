use std::collections::HashMap;
use std::io::{stdout, Write};
use std::fs;
use std::env;
use rand::{Rng, thread_rng};

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name: &str = match args.len() {
    2 => &args[1],
    _ => "./word",
  };

  let mut collect_count = 0;
  let mut play_count = 0;
	let mut wrong_words = Vec::new();

  let words_file = fs::read_to_string(file_name).expect("単語ファイルの読み込みに失敗しました。");
  let words_vec: Vec<&str> = words_file.split('\n').collect();  // ワード一覧
  let mut rng = thread_rng(); // 乱数発生源

  let mut renew_count = 0;

  loop {
    play_count += 1;
    let mut turn = 10;    // 残りのターン
    let mut input_char = HashMap::new();  // 入力した文字
    let mut raw_target = words_vec[rng.gen_range(0..=words_vec.len())].to_string();   // ターゲット
    raw_target.make_ascii_lowercase();
    let target = raw_target;

    if renew_count > words_vec.len() {
      panic!("適正な単語の取得に失敗しました");
    }

    // 適正な単語でなければ再生成
    if !word_check(&target) {
      renew_count += 1;
      continue;
    } else {
      renew_count = 0;
    }
    
    // ゲームのメインループ
    while turn > 0 {
      // print process
      println!("");
      print_word_and_usedch(&target, &input_char);
      println!("残り回数：{}", turn);
    
      // 入力が0文字ならもう一度
			// それ以外なら先頭を取得
      print!("please input>>");
      let ch = read_line();
      if ch.len() < 1 {
        continue;
      }
			let ch = ch.chars().next().expect("to_charに失敗しました。").clone();

      // 初めての入力なら１をセット、それ以外なら1を足す
      if input_char.contains_key(&ch) {
        input_char.insert(ch.clone(), input_char.get(&ch).unwrap()+1);
      } else {
        input_char.insert(ch.clone(), 1);
      }

			// すでに入力済み
      if input_char.get(&ch).unwrap().clone() != 1 {
        continue;
      }

			// 入力された文字が目標の単語に含まれていたら
      if !target.contains(&ch.to_string()) {
        turn -= 1;
      }

			// 正解の場合は終了
      if is_collect(&target, &input_char) {
        println!("Collect!!");
        collect_count += 1;
        break;
      }

      // ターンを使い果たして終了
      if turn == 0 {
        println!("You lose...");
        println!("The answer is {}.", target);
				wrong_words.push(target);
        break;
      }
    }	// end of ゲームループ

    let continue_flag;
    let mut yesorno;

    // 続けますかループ
    loop {
      print!("続けますか？y/n>");
      yesorno = read_line();

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
      println!("あなたのスコア\nwin:{}\nlose:{}\nrate:{}%", collect_count, play_count-collect_count, (collect_count as f32)/(play_count as f32));
			println!("間違えた単語：");
			for word in wrong_words {
				println!("- {}", word);
			}
      break;
    }
  }
}

fn read_line() -> String {
	stdout().flush().unwrap();
	let mut ch = String::new();
	std::io::stdin().read_line(&mut ch).expect("標準入力の読み込みに失敗しました。");
	ch = ch.trim().to_string();
	ch
}

fn is_collect(target: &str, input_char: &HashMap<char, u32>) -> bool {
	for ch in target.chars() {
		if !input_char.contains_key(&ch) {
			return false;
		}
	}
	true
}

// target 目標の単語, input_char これまでに入力された文字
fn print_word_and_usedch(target: &str, input_char: &HashMap<char, u32>) {
  let mut chars = target.chars();
  print!("使われた文字：");
  for ch in input_char.keys() {
    print!("{}", ch);
  }

	// まだ正解していないアルファベットは_
	// 正解しているアルファベットは表示する
  print!("\n単語：");
  for _ in 0..target.len() {
    let next = chars.next().unwrap().clone();
    if input_char.contains_key(&next) {
      print!("{}", next);
    } else {
      print!("_");
    }
  }
  println!("");
}

fn word_check(word: &str) -> bool {
  // 長すぎず、短すぎず
  // ascii以外の文字が含まれていない
  if word.len() < 3 || word.len() > 10 || !word.is_ascii() {
    return false;
  }

	let abcz = "abcdefghijklmnopqrstuvwxyz";
  // アルファベット以外が含まれない
  for ch in word.chars() {
    if !abcz.contains(&ch.to_string()) {
      return false;
    }
  }
  true
}

#[cfg(test)]
mod tests {
	use super::*;
	
#[test]
	fn is_collect_test() {
		let target = &"hello";
		let target2 = &"world";
		let mut hash = HashMap::new();
		for ch in target.chars() {
			hash.insert(ch.clone(), 1);
		}

		assert!(is_collect(target, &hash));
		assert!(!is_collect(target2, &hash));
	}

#[test]
  fn word_check_test() {
    let word1 = &"hello";
    let word2 = &"hello!";
    let word3 = &"hello1";
    let word4 = &"hl";
    let word5 = &"helloworldhogefugapiyo";
    let word6 = &"こんにちは";
    
    assert!(word_check(word1));
    assert!(!word_check(word2));
    assert!(!word_check(word3));
    assert!(!word_check(word4));
    assert!(!word_check(word5));
    assert!(!word_check(word6));
  }
}
