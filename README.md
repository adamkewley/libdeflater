# rlibdeflate

Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate)

Libdeflate is a block-based compressor that requires supplying the the
whole input/output buffer up-front. It is a simple,
performance-focused compression library that is typically used in
genomic [BAM](https://samtools.github.io/hts-specs/SAMv1.pdf) files.


# Compression Performance (Calgary Corpus)

Single-threaded compression performance 

```
bench     size [KB]    flate2 [us]    libdeflate [us]    speedup
----------------------------------------------------------------
paper1    53           112725         84636              1.3
progc     39           71801          56379              1.3
trans     93           130350         95235              1.4
obj1      21           57703          55165              1.0
pic       513          528318         219402             2.4
paper2    82           224239         77545              2.9
book1     768          376075         849271             0.4
progl     71           129568         90734              1.4
geo       102          389463         71096              5.5
obj2      246          331353         160858             2.1
news      377          267790         303038             0.9
bib       111          276025         91757              3.0
progp     49           76474          55564              1.4
book2     610          381831         567885             0.7
```
