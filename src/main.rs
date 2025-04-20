use std::fs::File;

use std::io::{BufReader,BufWriter,Write,Read,Lines,read_to_string};
use std::vec;

/// 定义关键字表
const reversed: &[&str] = &["auto","break","case",
	"char","const","continue","default","do","double","else",
	"enum","extern","float","for","goto","if","int","long","register","return",
	"short","signed","sizeof","struct","switch","typedef","union","unsigned",
	"void","volatile","while"];

/// 定义运算符表
const operator_fore: &[char] = &['+','-','*','/','=','<','>']; // 注意有 == 


/// 定义分隔符表

const splitter: &[char] = &['(',')','{','}','\n'];

fn main()
{
	// 定义标识符表
	let mut identifier:Vec<String> = vec![];
	// 定义常量表
	let mut number:Vec<i32> = vec![];

	let source = File::open("source.lsl").expect("err reading file");
	let text = read_to_string(source).expect("err read to string");

	let mut iter = text.chars();
	let mut c = iter.next().expect("err at start");
	while c != '#' {
		// println!("text:{:?}",iter.as_str());
		// println!("char:{:?}",c);
		if c.is_alphabetic() {
			let mut temp = String::new();
			while c.is_alphabetic() {
				temp.push(c);
				c = iter.next().expect(&format!("标识符定义异常：{}",temp));
			}
			// 此时开始查表
			if reversed.contains(&temp.as_str()) {
				// 生成保留字属性字
				let attribute = reversed.iter().position(|&x| x==temp.as_str()).expect("不可能到达词法分析器的这里！");
				println!("{}:<保留字,{}>",temp,attribute);
			} else {
				// 生成标识符属性字
				if identifier.contains(&temp) {
					let attribute = identifier.iter().position(|x|x==&temp).expect("???");
					println!("{}:<标识符,{}>",temp,attribute);
				} else {
					identifier.push(temp.clone());
					let attribute = identifier.len() - 1;
					println!("{}:<标识符,{}>",temp,attribute);
				}
			}
		} else if c.is_numeric() {
			let mut temp: i32 = 0;
			while c.is_numeric() {
				temp = temp * 10;
				temp = temp +  c.to_digit(10).expect("convert 失败") as i32;
				c = iter.next().expect("???");
			}
			// 生成常数属性字
			if number.contains(&temp) {
				let attribute = number.iter().position(|x|x==&temp).expect("???");
				println!("{}:<常数,{}>",temp,attribute);
			} else {
				number.push(temp.clone());
				let attribute = number.len() - 1;
				println!("{}:<常数,{}>",temp,attribute);
			}
		} else if operator_fore.contains(&c) || splitter.contains(&c) {
			if c=='=' && iter.clone().peekable().peek().unwrap()!=&' ' {
				if let Some(x) = iter.next() {
					if x == '=' {
						// 生成属性字
						println!("==:<运算符,0>");
					}else {
						// error
						panic!("={}:不是运算符",x);
					}
				}
			} else {
				// 生成属性字
				if operator_fore.contains(&c) {
					// 运算符属性字
					let mut attribute = operator_fore.iter().position(|x|x==&c).expect("???");
					attribute += 1; // 0 被 == 占用
					println!("{}:<运算符,{}>",c,attribute);
				} else {
					let attribute = splitter.iter().position(|x|x==&c).expect("???");
					if c == '\n' {
						println!("\\n:<分隔符,{}>",attribute);
					} else {
						println!("{}:<分隔符,{}>",c,attribute);
					}
				}
				c = iter.next().expect("无结束符号！");
			}
		} else if !c.is_control() && !c.is_whitespace() {
			// error
			panic!("有错误：{:?}",c);
		} else {
			c = iter.next().expect("无结束符号！");
		}
	}


	let mut buffer_reversed = File::create("table_reversed").expect("create reversed table err");
	let mut buffer_operator = File::create("table_operator").expect("create operator table err");
	let mut buffer_splitter = File::create("table_splitter").expect("create splitter table err");
	let mut buffer_identifier = File::create("table_identifier").expect("create identifier table err");
	let mut buffer_number = File::create("table_number").expect("create number table err");

	let reversed_data = reversed.join(" ");
	buffer_reversed.write(reversed_data.as_bytes()).expect("write reveresed table err");

	let operator_data = operator_fore.iter().map(|x|String::from(*x));
	let arr:Vec<String> = operator_data.collect();
	let msg = arr.join(" ") + " ==";
	buffer_operator.write(msg.as_bytes()).expect("write operator table err");

	let splitter_data = splitter.iter().map(|x|String::from(*x));
	let arr:Vec<String> = splitter_data.collect();
	let msg = arr.join(" ");
	buffer_splitter.write(msg.as_bytes()).expect("write splitter table err");

	let identifier_data = identifier.join(" ");
	buffer_identifier.write(identifier_data.as_bytes()).expect("write identifier table err");

	let number_data = number.iter().map(|x|x.to_string());
	let arr: Vec<String> = number_data.collect();
	let msg = arr.join(" ");
	buffer_number.write(msg.as_bytes()).expect("write number table err");

}
