param (
	$day,
	[switch]
	$Test
)

Push-Location "days/day-$day"

$inputFile = "input.txt"

if ($test -eq $true) {
	$inputFile = "input_test.txt"
}


Get-Content input.txt | cargo run

Pop-Location