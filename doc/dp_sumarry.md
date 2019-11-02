### 自己关于 dynamic programming 算法的一些思考：

在[知乎](https://www.zhihu.com/question/23995189)上的几个答案

_是对问题状态的定义和状态转移方程的定义_

#### dp 的几个概念：

- 无后效性：如果给定某一阶段的状态，则在这一阶段以后过程的发展不受这阶段以前各段状态的影响
- 最优子结构：大问题的最优解可以由小问题的最优解推出

所以 `能将大问题拆成几个小问题，且满足无后效性、最优子结构性质` 的问题就能通过 dp 来解决。

#### dp 的核心思想：

dp 自带剪枝，尽量缩小可解空间，枚举有希望成为答案的解答

#### dp 的阶段和状态

每次操作产生一个阶段，每个阶段可能会有多个状态，不同状态数量不必相同，一个阶段的一个状态可以的到下个阶段所有状态中的几个。

一个阶段的最优解可以中之前`若干个阶段`(1个或者多个)的最优状态中得到。

- 多个阶段一个状态的例子：
  ---
  
    _这种解法的时间复杂度和需要对比阶段的个数 `phase` 有关，一般为 `n * phase`，空间复杂度一般为 `n`_
    
    - [n0005_longest_palindromic_substring](../src/exercises/n0005_longest_palindromic_substring.rs): 
    ```
    状态推导：
        seq[i] == seq[j] && dp[j+1][i-1]
        
    遍历顺序：
    这里遍历的顺序和 516[] 不一样是因为需要的已知的状态只有一个
    由于确定了第一个遍历的值为 i , 所以 i 是需要从小到大的，而对于 j 来说，因为 i-1 的所有值在上一轮已经全部遍历成功，所以 j 的遍历顺序就无所谓了。
    ```
    
    - [n0300_longest_increasing_subsequence](../src/exercises/n0300_longest_increasing_subsequence.rs): 每个阶段只有一个状态，但是要获取这个状态的最优解需要去和之前的`每个阶段`的状态做比较。条件: `num[j] >= num[j]`
    
    - [n0279_perfect_squares](../src/exercises/n0279_perfect_squares.rs): 每个阶段也是一个状态，但是要和满足下述条件的阶段做比较来获取最优解。
    
      ```
      for j in 1..=(square-1) {
      	dp[i] = i32::min(dp[j*j] + dp[i - j*j], dp[i]);
      }
      ```
    - [0343_integer_break](../src/exercises/n0343_integer_break.rs): 和之前符合条件的阶段进行相乘以得到最优解。
    
    - [n0368_largest_divisible_subset](../src/exercises/n0368_largest_divisible_subset.rs): condition: `nums[j] % nums[i] == 0` 这个稍微有点区别，其他的只要求最优解的个数，这个要求返回最优解的具体内容，所以用链表存了最优解。

    - [n0416_partition_equal_subset_sum](../src/exercises/n0416_partition_equal_subset_sum.rs): 这个题目的本质就是某个数组能否拼凑成指定的某个值。
    
    - [n0494_target_sum](../src/exercises/n0494_target_sum.rs): 还是先分析问题 看出来的差不多还是某个数组能否拼凑出指定数值的问题。
    
    - [n0494_target_sum](../src/exercises/n0474_ones_and_zeroes.rs): 这个问题的状态推导 dp[m][n] = max(dp[m][n], dp[m - num_zeros][n - num_ones] + 1)。
    
    - [n0516_longest_palindromic_subsequence](../src/exercises/n0516_longest_palindromic_subsequence.rs): 这个问题的状态推导 
    ```
    if s[i] == s[j] {
        dp[i][j] = dp[i+1][j-1] + 2
    } else {
        dp[i][j] = max(dp[i+1][j], dp[i][j-1])
    }
    遍历顺序：
    因为这里推导出这个状态需要两个已知状态，与本状态相比，除了一个已知固定的下标，另外的下标分别为 dp[i+1] 和 dp[j-1]
    因为这里涉及到 dp[i+1] 这个状态，所以 i 是从大到小取值，而 dp[j-1] 这个状态，j 只需要从小到达取值。
    与第五道题目有点类似。
    ```
    
以上几个都是同一类问题，`dp[n]` 阶段需要和之前或者之后的每个阶段 `dp[i] (0<i<n或者是n<i<len)` 进行比较以得到最优解。


    - [221_Maximal Square](../src/exercises/n0221_maximal_square.rs): 这里的时间复杂度是 `n2` 是因为这里的操作对象是一个二维数组， 这里比较了之前的三个阶段，`dp[i-1][i-1]` `dp[i-1][i]` `dp[i][i-1]`, 对于时间复杂度的影响是常数级别的，空间复杂度也是 `n2`，但是可以优化，因为只需要上述三个阶段的值，对于 `dp[i-2]` 以及更早的结果都不需要了，所以空间复杂度可以优化到 `n`。
---


