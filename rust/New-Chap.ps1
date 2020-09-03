param ($packageName, [switch] $Lib, [switch] $Code);

if ($Lib) {
  cargo new $packageName --lib ;
}
else {  
  cargo new $packageName ;
}

Set-Location $packageName ;

"{
  `"watch`": [`"src/*.rs`", `"Cargo.toml`"],
  `"ext`": `"rs,toml`",
  `"ignore`": [`".git`"],
  `"exec`": `"cls && cargo fmt && cargo run`"
}" | Out-File .\nodemon.json
 

if ($Code) {
  code .
}