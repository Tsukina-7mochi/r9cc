#!/bin/bash
asset_print_foo="tmp_print_foo.o"

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

assert_stdout() {
    expected="$1"
  input="$2"

  echo "$input" | ./target/debug/r9cc > tmp.s
  cc -o tmp tmp.s $asset_print_foo
  actual="$(./tmp)"

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
cc -o "$asset_print_foo" -c "test_assets/print_foo.c"

assert 10 " 10 ; "
assert 41 " 12 + 34 -  5 ; "
assert 6 "3 * 4 / 2;"
assert 7 " 1 + 2 * 3; "
assert 9 "(1 + 2) * 3;"
assert 12 "3 * +4;"
assert 8 "3 * -4 + 20;"
assert 1 "1 < 2;"
assert 0 "2 < 1;"
assert 1 "2 > 1;"
assert 0 "1 > 2;"
assert 1 "1 <= 2;"
assert 1 "1 <= 1;"
assert 0 "2 <= 1;"
assert 1 "2 >= 1;"
assert 1 "1 >= 1;"
assert 0 "1 >= 2;"
assert 1 "1 == 1;"
assert 0 "1 == 2;"
assert 1 "1 != 2;"
assert 0 "1 != 1;"
assert 1 "a = 1;"
assert 1 "a = 1; a;"
assert 3 "a = 1; b = a + 1; b + 1;"
assert 3 "foo = 1; bar = 2; foo + bar;"
assert 1 "return 1; return 2; 3;"
assert 2 "if(1 < 2) 1; 2;"
assert 1 "if(1 < 2) 1; else 2;"
assert 10 "i = 0; while(i < 10) i = i + 1; i;"
assert 55 "sum = 0; i = 0; while(i <= 9) sum = sum + (i = i + 1); sum;"
assert 10 "for(i = 0; i < 10; i = i + 1) i; i;"
assert 55 "sum = 0; for(i = 1; i <= 10; i = i + 1) sum = sum + i; sum;"
assert 55 "sum = 0; i = 1; while(i <= 10) { sum = sum + i; i = i + 1; } sum;"
assert_stdout "foo" "print_foo();"

rm "$asset_print_foo"

echo OK
