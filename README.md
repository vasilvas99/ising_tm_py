# Parallel 2D Ising TM matrix assembler


## Installation:

1. Download the lastest build:

https://nightly.link/vasilvas99/ising_tm_py/workflows/CI/main/wheels.zip

2. Unzip the wheels.zip archive

3. Open a terminal in the unzipped directory and run:

```shell
pip install --force-reinstall --no-index --find-links="." ising_tm_py
```


## Usage

Now the library is installed as a global python package. You can now import it for any python file. To see the module documentation open a terminal and run

```shell
$ python -c "import ising_tm_py; help(ising_tm_py)"
Help on package ising_tm_py:

NAME
    ising_tm_py

DESCRIPTION
    Calculates the 2D Ising transfer matrix directly (no-BD) in parallel
    n - int - number of spins in the chain
    temp - non-dimensionalized temperature.

PACKAGE CONTENTS
    ising_tm_py

FUNCTIONS
    generate_ising_tm(n, jnn, temp)
        Calculates the 2D Ising transfer matrix directly (no-BD) in parallel
        n - int - number of spins in the chain
        temp - non-dimensionalized temperature.

DATA
    __all__ = ['generate_ising_tm']

FILE
    c:\python310\lib\site-packages\ising_tm_py\__init__.py
```

So the only function for now is the generate_ising_tm function.


```python
from ising_tm_py import generate_ising_tm

# Generate the 2^8 * 2^8 ising TM-matrix with coupling constant jnn = -1.0
# and non-dimensionalized temperature of 2.0
# The function returns a numpy 2D-ndarray (matrix).
tm = generate_ising_tm(n=8, jnn=-1.0, temp=2.0) 
```
