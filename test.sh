#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  echo "$input" | ./target/debug/r9cc > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi

  rm ./tmp
  rm ./tmp.s
}

cargo build

assert 0 0
assert 42 42

echo OK
