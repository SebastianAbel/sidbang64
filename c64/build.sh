#!/bin/bash

echo '>> cleanup'
rm ./release/main.prg
rm ./main.sym

echo '>> kickass'
java -jar ./KickAssembler/KickAss.jar main.asm -o release/main.prg

#echo '>> x64'
#cd ./release
#x64  main.prg
