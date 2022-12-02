param (
	$day
)

Push-Location "days/day-$day"

Get-Content input.txt | cargo run

Pop-Location