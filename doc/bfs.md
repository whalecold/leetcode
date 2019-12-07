### 广度优先算法（常被用于计算最短路径等问题）

- 广度优先算法常配合队列使用
  + 先把 root 存放到 queue 中，记录 root 元素的个数，这几个 root 记作一个 level。
  + 开始遍历，`queue.pop_front` 所有的 root 元素，然后把他们所有指向下一个 level 的元素 `queue.push_back`, 记录下一个 level 的总个数。
  + 第二部一直循环一直到 `queue.is_empty`。
- 对应的[深度优先算法](./dfs.md)配置栈使用的比较多。

#### [jump_game_ii](../src/exercises/n0045_jump_game_ii.rs):
- 第一次做这道题目的时候，想的是暴力解法, 从后往前计算出每个 index 的 min step，
然后在 `index - 1`  的地方去遍历 `[1,num[index - 1]]` 的最小去加一赋值给 `min_step[index-1]`
- 上面的做法挺蠢的，然后看了别人的解法，是一个 `implicit bfs`.
jumps 表示层数，farthest 表示当前层可以到达最远的地方， cur_end 表示这层的结尾。
```
 i == curEnd means you visited all the items on the current level. 
 Incrementing jumps++ is like incrementing the level you are on. 
 And curEnd = curFarthest is like getting the queue size (level size) 
 for the next level you are traversing.
```

#### [word_ladder](../src/exercises/n0127_word_ladder.rs):

- 标准的 bfs 解法，用一个队列存放元素，然后从第每层开始找到所有符合条件的
元素 `push_back`，并且记录下一层元素的个数 n，这层遍历完成后，根据之前记录的这层个数
遍历 n 次，每次遍历的时候把查完的元素 `pop_front`，以此往复，知道队列为空或者找到了 end_word
为止。
