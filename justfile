new day:
  echo "creating scaffold for day {{day}}"
  touch ./inputs/day_{{day}}
  touch ./inputs/day_{{day}}.example
  touch ./src/day_{{day}}.rs
  touch ./examples/day_{{day}}.rs
  touch ./benches/day_{{day}}.rs
