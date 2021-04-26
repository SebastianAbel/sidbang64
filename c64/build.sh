#!/bin/bash

echo '>> cleanup'
rm ./a.out
rm ./release/main.prg
rm ./main.sym

echo '>> kickass'
java -jar ./KickAssembler/KickAss.jar main.asm -o release/main.prg

#echo '>> x64'
../../exomizer-3.1.0/bin/exomizer sfx sys ./release/main.prg
rm ./release/main.prg
mv ./a.out ./release/main.prg

#x64  main.prg
