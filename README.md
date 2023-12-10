# Advent of Code
My rust solution of advent of code.

## How to use
You need to grab the `session` cookie and set the value in the `SESS` environment variable.
Currently, the year is hard coded in source code and may change to pass in environment variable in the future.

Pass the date you want to solve as first argument, for example, solve problem of day 4 
```bash
./aoc 4
```
Testcases are downloaded automatically.

If you want to use your own testcase for test purpose, add `test` before date in command line argument, 
for example, 
```bash
./aoc test4
```
to test problem of day 4.

## WIP
Make this also a library that can help other write their own Advent of Code solution.