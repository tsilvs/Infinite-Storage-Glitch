# Dislodging Benchmark Notes

#### <b> All of the benchmarks were performed on an M1 MacBook Pro with 16 GB of RAM. </b>

This is a comparison between multithreaded and single-threaded dislodging. The benchmark file size is approximately 155 MB, and it increases to 682 MB when embedded.

Number of frames: 5551

## Single-Threaded Benchmarks

    221429ms -> 3.69 minutes
    222202ms -> 3.7 minutes

## Multithreaded Benchmarks

#### Batch size of 30:

    51516ms -> 0.86 minutes
    51425ms -> 0.86 minutes

#### Batch size of 20:

    55602ms -> 0.92 minutes
    55702ms -> 0.93 minutes

#### Batch size of 40:

    54083ms -> 0.9 minutes
    53990ms -> 0.89 minutes

A batch size of 30 images per thread appears to be the most optimal.

## Read Frames per Second

#### Multithreaded:

$\frac{5551}{51516ms / 1000} =  107.8fps$

#### Single-Threaded:

$\frac{5551}{221429ms / 1000} =  25.1fps$

This represents an increase of 4.3x.
