# binary search text files

$ time grep -m 1 '^venulose' words.txt
venulose

real	0m7.116s
user	0m7.048s
sys	0m0.057s
$ time grep -m 1 '^venulooooooser' words.txt

real	0m7.637s
user	0m7.519s
sys	0m0.059s
$ time bgrep venulose words.txt
Found it at 242710635

real	0m0.016s
user	0m0.002s
sys	0m0.004s
$ time bgrep venulooooooser words.txt
Not found

real	0m0.006s
user	0m0.002s
sys	0m0.003s
