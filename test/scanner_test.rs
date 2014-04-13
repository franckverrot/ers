extern crate ers;

use std::io::BufferedReader;
use std::io::fs::File;
use std::str::eq;
use ers::{Scanner};

fn fail_with_file(path : ~str) {
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();

  let mut s = Scanner::new(peekable, path.clone());

  match s.scan() {
    None => assert!(true),
    Some(b) => assert!(false, b.content)
  }
}

#[test]
fn test_scanner_scan_text_block() {
  let path = ~"test/fixtures/scan_text_block.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s = Scanner::new(peekable, path.clone());
  let b = s.scan().unwrap();

  assert!(std::str::eq(&b.content, &~"hello world\n"));
  assert!(b.pos.line_no == 2);
}

#[test]
fn test_scanner_scan_text_block_single_lt() {
  let path = ~"test/fixtures/single_lt.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s = Scanner::new(peekable, path.clone());
  let b = s.scan().unwrap();

  assert!(std::str::eq(&b.content, &~"<\n"));
}

#[test]
fn test_scanner_scan_text_block_starting_lt() {
  let path = ~"test/fixtures/starting_lt.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s    = Scanner::new(peekable, path.clone());
  let b = s.scan().unwrap();

  assert!(std::str::eq(&b.content, &~"<html>\n"));
}

#[test]
fn test_scanner_scan_code_block() {
  let path = ~"test/fixtures/code_block.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s    = Scanner::new(peekable, path.clone());
  let b = s.scan().unwrap();

  assert!(std::str::eq(&b.content, &~" let x = 1 "), b.content)
}

#[test]
fn test_scanner_scan_code_block_unexpected_eof_1() {
  fail_with_file(~"test/fixtures/code_block_unexpected_eof_1.ers");
}

#[test]
fn test_scanner_scan_code_block_unexpected_eof_2() {
  fail_with_file(~"test/fixtures/code_block_unexpected_eof_2.ers");
}

#[test]
fn test_scanner_scan_code_block_unexpected_eof_3() {
  fail_with_file(~"test/fixtures/code_block_unexpected_eof_3.ers");
}

#[test]
fn test_scanner_scan_code_block_unexpected_eof_4() {
  fail_with_file(~"test/fixtures/code_block_unexpected_eof_4.ers");
}

#[test]
fn test_scanner_scan_header_block() {
  let path = ~"test/fixtures/header_block.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s = Scanner::new(peekable, path.clone());
  let b = s.scan().unwrap();

  assert!(std::str::eq(&b.content, &~" use std::io "), b.content);
}

#[test]
fn test_scanner_scan_header_block_unexpected_eof_1() {
  fail_with_file(~"test/fixtures/header_block_unexpected_eof_1.ers");
}

#[test]
fn test_scanner_scan_header_block_unexpected_eof_2() {
  fail_with_file(~"test/fixtures/header_block_unexpected_eof_2.ers");
}

#[test]
fn test_scanner_scan_header_block_unexpected_eof_3() {
  fail_with_file(~"test/fixtures/header_block_unexpected_eof_3.ers");
}

#[test]
fn test_scanner_scan_header_block_unexpected_eof_4() {
  fail_with_file(~"test/fixtures/header_block_unexpected_eof_4.ers");
}

#[test]
fn test_scanner_scan_header_block_unexpected_eof_5() {
  fail_with_file(~"test/fixtures/header_block_unexpected_eof_5.ers");
}

#[test]
fn test_scanner_scan_print_block() {
  let path = ~"test/fixtures/print_block.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s = Scanner::new(peekable, path.clone());
  let b = s.scan().unwrap();

  assert!(std::str::eq(&b.content, &~" myNum "), b.content);
}


#[test]
fn test_scanner_scan_print_block_unexpected_eof() {
  fail_with_file(~"test/fixtures/print_block_unexpected_eof.ers");
}

#[test]
fn test_scanner_scan_eof() {
  fail_with_file(~"test/fixtures/eof.ers");
}

#[test]
fn test_scanner_scan_multiline() {
  let path = ~"test/fixtures/multiline.ers";
  let mut buf  = ~BufferedReader::new(File::open(&Path::new(path.clone()))) as ~Buffer;
  let input    = buf.read_to_str().unwrap();
  let peekable = input.chars();
  let mut s = Scanner::new(peekable, path.clone());

  match s.scan() {
    None => assert!(false),
    Some(b) => {
      assert!(b.class == ers::blocks::Text);
      assert!(std::str::eq(&~"hello\\nworld", &b.content));
    }
  }
  match s.scan() {
    None => assert!(false),
    Some(b) => {
      assert!(b.class == ers::blocks::Print);
      assert!(std::str::eq(&~" x \n \n\n", &b.content));
    }
  }
  match s.scan() {
    None => assert!(false),
    Some(b) => {
      assert!(b.class == ers::blocks::Text);
      assert!(std::str::eq(&~"goodbye\n", &b.content));
    }
  }
}
