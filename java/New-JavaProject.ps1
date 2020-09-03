param ($packageName, [switch] $Lib, [switch] $Code)

mkdir $packageName
Set-Location $packageName

if ($Lib) {
    gradle init --dsl groovy --type java-library --test-framework junit
}
else {
    gradle init --type java-application --dsl groovy --test-framework junit
}

gradle -t run

