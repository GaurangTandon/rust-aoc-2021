# rust-aoc-2021

I hope I don't procrastinate this time.
Learning Rust via AdventOfCode 2021, one day at a time.

## Rules on sharing solution

It is fine to share solutions for which you don't receive points on the global leaderboard, which is after it has crossed first 100 participants. See [official FAQ](https://adventofcode.com/2021/about).

## Downloading input using script

### Set session cookie

You need to:

```
cd aoc
cp cookie.env.template cookie.env
```

And then edit the `cookie.env` with your cookie from the website. You can find the cookie in the Browser DevTools in the Storage tab.

This cookie will change if you login/logout of the website.

### Run script

Specify day number and run the script.

```
cd aoc
./get-input.sh 1 # for day 1
```

Input is saved in `aoc/inputs/X.txt`.
