param ($packageName, [switch] $Lib, [switch] $Code, [switch] $Run)

mkdir $packageName
Set-Location $packageName

if ($Lib) {
    gradle init --dsl groovy --type kotlin-library --test-framework kotlintest
}
else {
    gradle init --type kotlin-application --dsl groovy --test-framework kotlintest
}

if ($Code) {
    code .
}

if ($Run) {
    gradle -t run
}
