param (
	$day
)

Push-Location "days/day-$day"

cat input.txt | cargo run

Pop-Location