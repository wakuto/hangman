use std::collections::HashMap;
use std::io::{stdout, Write};
use std::fs;
use std::env;
use rand::{Rng, thread_rng};

#[derive(Clone, Copy, PartialEq)]
/// ゲームのモードを表します
/// * `Normal` - 通常のモード
/// * `Poor` - 苦手克服モード
enum Mode {
  Normal,
  Poor,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name: &str = match args.len() {
    2 => &args[1],
    _ => "./word",
  };

  let mut collect_count = 0;
  let mut play_count = 0;
	let mut wrong_words = HashMap::new();

  let words_file = fs::read_to_string(file_name).expect("単語ファイルの読み込みに失敗しました。").to_lowercase();
  let mut words_vec: Vec<&str> = words_file.split('\n').collect();  // ワード一覧
  word_initialize(&mut words_vec);
  if words_vec.len() == 0 {
    println!("単語数が少なすぎます");
    return;
  }
  let mut rng = thread_rng(); // 乱数発生源

  // 苦手ワードの読み込み
  let poor_file = match fs::read_to_string("./poor_word") {
    Ok(word) => word.to_lowercase(),
    _ => String::new(),
  };
  let mut poor_words: Vec<&str> = match poor_file.len() {
    0 => Vec::new(),
    _ => poor_file.split('\n').collect(),
  };
  word_initialize(&mut poor_words);

  loop {
    // mode select
    let mut mode: Option<Mode>  = match poor_words.len() {
      0 => Some(Mode::Normal),
      _ => None,
    };
    while mode == None {
      println!("通常モード: 1, 苦手克服モード: 2");
      print!(">");
      let m = read_line();
      if m.len() == 1 {
        mode = match m.chars().nth(0).unwrap() {
          '1' => Some(Mode::Normal),
          '2' => Some(Mode::Poor),
          _ => None,
        };
      }
    }
    let mode = mode.unwrap();

    play_count += 1;
    let mut turn = 10;    // 残りのターン
    let mut input_char: HashMap<char, bool> = HashMap::new();  // 入力した文字
    let target = match mode {
      Mode::Normal => words_vec[rng.gen_range(0..words_vec.len())].to_string(),
      Mode::Poor => poor_words[rng.gen_range(0..poor_words.len())].to_string(),
    };

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

      // 入力がアルファベットでなければ戻る
      if !is_alpha(&ch.to_string()) {
        continue;
      }

      // 初めての入力なら１をセット、それ以外なら1を足す
      if input_char.contains_key(&ch) {
        continue;
      }
      input_char.insert(ch.clone(), true);

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
        wrong_words.insert(target.clone(), true);
        break;
      }
    }	// end of ゲームループ

    let mut continue_flag = None;
    let mut yesorno;

    // 続けますかループ
    while continue_flag == None {
      print!("続けますか？y/n>");
      yesorno = read_line();

      // yならつづける、nなら終わる。 それ以外はもう一度読み取り
      if yesorno.len() == 1 {
        continue_flag = match yesorno.chars().nth(0).unwrap() {
          'y' => Some(true),
          'n' => Some(false),
          _ => None,
        };
      }
    }
    if !continue_flag.unwrap() {
      println!("あなたのスコア");
			println!("win:{}", collect_count);
			println!("lose:{}", play_count - collect_count);
			println!("rate:{}%", 100.0*(collect_count as f32)/(play_count as f32));
			println!("間違えた単語：");
      let mut wrong_string = String::new();
			for word in wrong_words.keys() {
				println!("- {}", word);
        wrong_string += &(String::from("\n") + word);
			}
      fs::write("poor_word", &wrong_string).expect("poor_wordの書き込みに失敗しました。");
      break;
    }
  }
}

/// リスト中の文字列のうちアルファベット以外が含まれるものと短すぎたり長すぎたりするものを削除します。
/// * `words` - 操作対象の文字列のリスト
fn word_initialize(words: &mut Vec<&str>){
  let mut remove_index = Vec::new();
  for i in 0..words.len() {
    let word = words[i].to_string();
    if !is_alpha(&word) || !word_check(&word) {
      remove_index.push(i);
    }
  }
  remove_index.reverse();
  for i in remove_index {
    words.remove(i);
  }
}

/// 標準入力から1行読み取って返却します。
fn read_line() -> String {
	stdout().flush().unwrap();
	let mut ch = String::new();
	std::io::stdin().read_line(&mut ch).expect("標準入力の読み込みに失敗しました。");
	ch = ch.trim().to_string();
	ch
}

/// 文字列の構成文字すべてがある文字集合に含まれるかを返します。
/// * `target` - 検査対象の文字列
/// * `input_char` - 文字集合 
fn is_collect(target: &str, input_char: &HashMap<char, bool>) -> bool {
	for ch in target.chars() {
		if !input_char.contains_key(&ch) {
			return false;
		}
	}
	true
}

// target 目標の単語, input_char これまでに入力された文字
/// 文字列のハッシュマップに含まれる構成文字のみを出力します。
/// * `target` - 出力対象の文字列
/// * `input_char` - 出力する文字
fn print_word_and_usedch(target: &str, input_char: &HashMap<char, bool>) {
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

/// 文字列が小文字アルファベットのみから構成されているかを返します。
/// * `word` - 検査対象の文字列
fn is_alpha(word: &str) -> bool {
	let abcz = "abcdefghijklmnopqrstuvwxyz";
  // アルファベット以外が含まれない
  for ch in word.chars() {
    if !abcz.contains(&ch.to_string()) {
      return false;
    }
  }
  true
}

/// 文字列が長すぎたり短すぎたりしないかを返します。
/// * `word` - 検査対象の文字列
fn word_check(word: &str) -> bool {
  // 長すぎず、短すぎず
  // ascii以外の文字が含まれていない
  if word.len() < 3 || word.len() > 10 {
    return false;
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
			hash.insert(ch.clone(), true);
		}

		assert!(is_collect(target, &hash));
		assert!(!is_collect(target2, &hash));
	}

#[test]
  fn word_check_test() {
    let word = &"hello";
    let word1 = &"hl";
    let word2 = &"helloworldhogefugapiyo";
    
    assert!(word_check(word));
    assert!(!word_check(word1));
    assert!(!word_check(word2));
  }

#[test]
  fn is_alpha_test() {
    let word1 = &"hello";
    let word2 = &"hello!";
    let word3 = &"hello1";
    let word4 = &"こんにちは";

    assert!(is_alpha(word1));
    assert!(!is_alpha(word2));
    assert!(!is_alpha(word3));
    assert!(!is_alpha(word4));
  }

#[test]
  fn word_initialize_test() {
    let mut vect = vec!["hello", "hello!", "he", "helloworldhogefuga", "こんにちは"];
    word_initialize(&mut vect);
    assert_eq!(vec!["hello"], vect);
  }

}
