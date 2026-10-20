## 构建比分表

给定的是一场足球比赛的比分列表（每行一个）。每行的格式为：
```
<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
```

示例：`England,France,4,2` （英格兰进了4个球，法国进了2个球）。

你需要构建一个比分表，其中包括球队名称、球队进球数和球队失球数。构建比分表的一种方法是使用 HashMap。解决方案已经部分使用了 HashMap，完成它以通过测试。

让我通过测试！

<div class="hint">
使用 <code>entry()</code> 和 <code>or_insert()</code> 方法在比分表中插入与每个球队对应的条目。

在[书籍](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value)中了解更多信息。
</div>

<div class="hint">
如果已存在给定键的条目，可以根据现有值更新 <code>entry()</code> 返回的值。

在[书籍](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value)中了解更多信息。
</div>