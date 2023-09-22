### Binary Search:
#### MID:
- There are two ways to find mid but one more reliable
  - mid = (start + end) / 2;
  - mid = start + (end - start) / 2;

- The first ways create issue when (start + end) `overflow` out of size usize
- second is same as fist 
  - `start + (end/2) - (start/2)`
  - = start - (start/2) + (end/2) 
  - = (2start - start + end)/2
  - = (start+end)/2



### LEFT:
- 724. Find Pivot Index


