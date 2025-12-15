# Tasks related to days

## new_day (day)
> Creates a new crate for a day.

```bash
if [ -d "day$day" ]; then
    echo "Crate for the day $day already exist"
    exit;
fi


echo "Creating new crate for day $day"
cargo generate --path ../day_template --name day$day
cd day$day
cargo add rayon criterion itertools
cd ..

echo "Opening the challenge in the browser"
firefox "https://adventofcode.com/2025/day/$day"

## Get AOC session key
source ../.env 

# Fetching the input for the challenge
curl -s --cookie "session=$SESSION" https://adventofcode.com/2025/day/$day/input -o day$day/input.txt &
echo "Enter test input" > day$day/test.txt
touch day$day/test_ans.txt
st bacon clippy day$day &
nvim day$day/test.txt
```

## test (day)
> Runs the tests for the given day.

```bash
cargo t -p day$day
```

## run (day)
> Runs the tests for the given day.

**OPTIONS**
* part
    * flags: -p --part
    * type: number
    * desc: Which part to run
    * required
    * choices: 1, 2

```bash
cargo r -p day$day -- $part
```

## bench (day)
> Perform benchmarking for the specified day.

```bash
cargo bench -p day$day
```
