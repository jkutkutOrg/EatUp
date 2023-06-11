#!/bin/bash

files2check="*.java *.tsx *.ts *.rs"

echo "Counting size of the project:"

for file in $(echo $files2check); do
    echo "Counting lines with files with pattern $file:"
    find . -name "$file" -exec cat {} \; | wc -l
done

echo "Nice!"