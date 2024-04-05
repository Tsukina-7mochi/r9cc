#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  echo "$input" | ./target/debug/r9cc > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "'$input' => '$actual'"
  else
    echo "'$input' => '$expected' expected, but got '$actual'"
    exit 1
  fi

  rm ./tmp
  rm ./tmp.s
}

cargo build

assert 10 " 10 "
assert 41 " 12 + 34 -  5 "
assert 6 "3 * 4 / 2"
assert 7 " 1 + 2 * 3 "
assert 9 "(1 + 2) * 3"
assert 12 "3 * +4"
assert 8 "3 * -4 + 20"
assert 1 "1 < 2"
assert 0 "2 < 1"
assert 1 "2 > 1"
assert 0 "1 > 2"
assert 1 "1 <= 2"
assert 1 "1 <= 1"
assert 0 "2 <= 1"
assert 1 "2 >= 1"
assert 1 "1 >= 1"
assert 0 "1 >= 2"
assert 1 "1 == 1"
assert 0 "1 == 2"
assert 1 "1 != 2"
assert 0 "1 != 1"

echo OK
