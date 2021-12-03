I have performed a memory optimized but time slow worst case O(nlog^2n) solution.  
But it was possible to solve this problem in memory worse but time efficient manners using tries or next pointers.  
Both allow us to converge at the correct solution in logn time.
Trie will take nlogn to build.  
Next pointers are `next[index][bit]` points to the last index in the array which has the same bit as this one. These next pointers will take `2*n` space to build and nlogn time.
