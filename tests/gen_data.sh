#!/bin/bash

set -eu
cd $(dirname $0)

function create_mismatched_png() {
    convert xc:white png.png
    mv png.png png.$1
}

function create_mismatched_jpg() {
    convert xc:white jpg.jpg
    mv jpg.jpg jpg.$1
}

function create_mismatched_gif() {
    convert xc:white gif.gif
    mv gif.gif gif.$1
}

function create_mismatched_pdf() {
    printf "%%PDF-1.3\n" > pdf.pdf
    mv pdf.pdf pdf.$1
}

mkdir data
cd data
convert xc:white png.png
create_mismatched_png jpg
create_mismatched_jpg pdf
echo "foo" >> ascii.txt
touch empty

mkdir dir1
cd dir1
create_mismatched_gif png
convert xc:white gif.gif
touch empty

mkdir dir2
cd dir2
create_mismatched_pdf png
echo "あ" >> unicode.txt
