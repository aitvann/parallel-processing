# Parallel Processing

Illustrates processing data in parallel utilazing full capacity of the host.

## Usage

``` sh
❯ cargo run --quiet 
number of cpus: 16
input data: [21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2]
seq no: 15, result 8, result seq no: 0, thread idx: Some(13)
seq no: 17, result 3, result seq no: 1, thread idx: Some(7)
seq no: 16, result 5, result seq no: 2, thread idx: Some(4)
seq no: 8, result 233, result seq no: 3, thread idx: Some(6)
seq no: 7, result 377, result seq no: 4, thread idx: Some(3)
seq no: 9, result 144, result seq no: 5, thread idx: Some(11)
seq no: 6, result 610, result seq no: 6, thread idx: Some(5)
seq no: 5, result 987, result seq no: 7, thread idx: Some(0)
seq no: 19, result 1, result seq no: 8, thread idx: Some(5)
seq no: 18, result 2, result seq no: 9, thread idx: Some(8)
seq no: 12, result 34, result seq no: 10, thread idx: Some(1)
seq no: 13, result 21, result seq no: 11, thread idx: Some(3)
seq no: 14, result 13, result seq no: 12, thread idx: Some(4)
seq no: 4, result 1597, result seq no: 13, thread idx: Some(10)
seq no: 10, result 89, result seq no: 14, thread idx: Some(14)
seq no: 3, result 2584, result seq no: 15, thread idx: Some(9)
seq no: 11, result 55, result seq no: 16, thread idx: Some(4)
seq no: 2, result 4181, result seq no: 17, thread idx: Some(15)
seq no: 1, result 6765, result seq no: 18, thread idx: Some(2)
seq no: 0, result 10946, result seq no: 19, thread idx: Some(12)
```
