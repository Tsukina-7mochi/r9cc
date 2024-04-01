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

echo OK
