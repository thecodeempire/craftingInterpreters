param ($packageName, [switch] $Code);

mkdir $packageName ;
Set-Location $packageName ;

"{
  `"watch`": [`"*.c`", `"*.h`"],
  `"ext`": `"c,h`",
  `"ignore`": [`".git`"],
  `"exec`": `"cls && .\\build.bat && .\\main.exe`"
}
" | Out-File .\nodemon.json ;

"*.exe" | Out-File .\.gitignore ;

# MakeFile 
"@echo off

gcc main.c -o main.exe
" | Out-File .\build.bat ;


# C Code 
"#include<stdio.h>

int main() {
    printf(`"Hello World \n`");
    return 0;
}
" | Out-File .\main.c ;

if ($Code) {
  code .
}