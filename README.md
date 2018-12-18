# Half Matrix
A Half Matrix implementation. Like a normal matrix, but you only store half of it. Also it only contains bools.

### Half Matrix: Storage
The matrix is row major.
Here's how it looks like:

```
 ABCD
A-
B--
C---
D----
```

In memory representation:
```
-|--|---|----
```


Indexing starts at 0.
```
ABCD
0123
```

Row Major means that the first parameter of the methods is the Y axis in the matrix, and the second is the X axis.

As shown by the matrix, the row value is required to be bigger or equal to the column value.

### Example
Parameters: (3, 0) = (D, A)
```
 ABCD
A-
B--
C---
DX---
```

### Contributing

If you see a missing method or a bug, open a pull request and I'll be more than happy to merge it!
