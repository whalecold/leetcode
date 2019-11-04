### 数组习题的思考：
------
#### [two_sum](../src/exercises/n0001_two_sum.rs):

先对数组进行排序，然后就是从两端开始不断的滑动坐标，直到接近需要达到的值。
但是！ 这里取的是下标，所以排序了之后下标会变，不能这么做，另外一个办法就是用一个 map 保存起来，然后遍历。

#### [median_of_two_sorted_arrays](../src/exercises/n0004_median_of_two_sorted_arrays.rs): 
首先问题的本质是把两个数组重新划分为两个部分，其中一部分永远小于另外的一部分，
这里可以先对数组 A 做划分，加入 A 的长度是 m:
```
        left_A        |    right_A
 A[0], A[1],...,A[i-1]| A[i],A[i+1],...,A[m-1]
```
  同样长度为 n 的数组 B 也做划分：
```
        left_B        |    right_B
 B[0], B[1],...,B[j-1]| A[j],A[j+1],...,A[n-1]   
```
这样左边的部分就是 left_A + left_B, 右边部分是 right_A + right_B, 这里可以推断出：
```
1、len(left) == len(right)
2、max(left) <= min(right)
```

#### [3sum](../src/exercises/n0015_3sum.rs):

这里的思路可以简化为 2sum, 这个题目取的是值，和下标没有关系，所以可以先进行排序，
所以在遍历的时候先保持第一个值不变，然后让剩余的两个数不断的进行滑动，
让滑动的值向目标值进行偏移直到不断的接近。如果要取值的话会麻烦不少，目前想到的办事还是按照 map
保存好数据。

#### [3sum_closest](../src/exercises/n0016_3sum_closest.rs):

和 3sum 基本抑制，还是先排序然后滑动窗口的思路，这里会去一个最接近的值，保存就好了

#### [4sum](../src/exercises/n0018_4sum.rs):

和 3sum 差不多，还是排序然后滑动窗口，这里要注意下 `unique`，所以在每次循环遍历的时候都需要
检查去重。

#### [next_permutation](../src/exercises/n0031_next_permutation.rs):

这题我也不会，但是按照这个[方法](https://leetcode.com/problems/next-permutation/discuss/13866/Share-my-O(n)-time-solution)来做就能解出来了，留待后面看看吧 不再花很多时间在上面了.

#### [search_in_rotated_sorted_array](../src/exercises/n0033_search_in_rotated_sorted_array.rs):

这道题目是要求时间复杂度为 O(log N) ，我的思路就是先用二分法找到 pivot，然后再以这个 pivot 分割数组，
用剩下的这部分数据进行二分法查找，总体时间复杂度算下来最差为 2O(log N)，符合题目的要求。

#### [find_first_and_last_position_of_element_in_sorted_array](../src/exercises/n0034_find_first_and_last_position_of_element_in_sorted_array.rs):

- 简单来说还是二分法，用两次，判断条件是 `nums[mid] >= target`
但是两次的条件不一样，如果在判断的时候加了 `=` 会找到
第一个匹配的值，不加等号会找到最后一个或者偏大的个值。
- 以前没有注意过一点就是 `left = mid + 1` 而 `right = mid`，自己思考了下大概是和
`(left + right) / 2` 有关系，因为这个得到的结果肯定是比真实的要小一点或者一样，
所以偏小的时候加 `+ 1` 也不会越界，而这时候如果 `- 1` 就肯定越界了。

#### [first_missing_positive](../src/exercises/n0041_first_missing_positive.rs):

- 首先要庆祝下！这是第一道自己独立完成的 `hard` 哈哈哈哈哈哈哈哈哈！！！
- 然后就是记录下自己的解题思路啦！
  + 首先看到题目的要求就是时间复杂度要求是 `O(n)` , 然后空间复杂堵是 `constant`. 但是本着先把题目
  做出来然后慢慢优化的态度，我就直接给这个数组做了排序，然后从非 0 的数值开始慢慢从 1 开始对比，这样当然是
  不对的，快排就是 `O(log n)`。
  + 然后我就想着用数组下标来表示数，`missing[i]` 来表示这个数是否存在, 为了要存下所有的数，所以就要找到最大值
   `max_num`，这里就遍历了一遍，时间复杂度是 `O(n)`，然后申请一个长度为 `max_num + 1` 的数组，再去遍历把 `nums` 中的所
   有以 `positive integer` 为下标的数置 1，这时候就遍历了第二遍，总体的时间复杂度就是 `2 O(n)` ，最后一步就是去遍历
   `missing` 数组，从 1 开始找到第一个非零的数，就是答案了，这个步骤看着时间复杂度可能会很高，但是时间上不可能会查到
   超过 `nums.len()` 的地方，所以最后总的时间复杂度就是 `3 O(n)`，满足了时间复杂度的要求。
  + 其实从上面的讨论也能看的出来，missing 的长度是可以直接设置为 `nums.len() + 1` 的，因为第一个不存在的正数肯定是在
  `1..nums.len()` 里面的，所以优化了之后时间复杂度就可以降为 `2 O(n)`, 空间复杂度降为 `O(n)`，但还是不满足 `constant` 的
  需求。
  + 这步就要看怎么把空间复杂度降下来了，如果想把空间复杂度降下来只有一个办法，就是去调整原来的 nums 的数组，
  就需要不断的把符合要求的数 `0 < num <= nums.len()` 放在对应的坐标上去。最后判断还是和原来差不多。

