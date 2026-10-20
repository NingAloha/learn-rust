## 统计已完成的作业

让这段代码可以编译！

想法是，在第12行生成的线程正在完成作业，而主线程正在监测进度，直到完成10个作业。由于生成的线程和等待线程之间的休眠时间不同，当您看到6行“正在等待…”并且程序结束而不超时时，就成功了 :)

<div class="hint">
  <code>Arc</code> 是一个原子引用计数指针，允许对<b>不可变</b>数据的安全共享访问。但是我们想要改变 <code>jobs_completed</code> 的数量，因此还需要使用另一种类型，以便每次只允许一个线程修改数据。
  请查看<a href ="https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct">本书的这个章节</a>，如果想要更多提示，请继续向下滚动 :)
</div>

<div class="hint">
  现在在 main 函数的开头是否有一个 <code>Arc</code> <code>Mutex</code> <code>JobStatus</code>？比如：

  
  <code>let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));</code>
  
  类似于书中示例代码发生在文本“我们可以使用 Arc\<T\> 来解决这个问题。”之后。
  如果没有，请尝试这样做！
  如果有了并且想要更多提示，请继续向下滚动！！
</div>

<div class="hint">
  确保您的线程都没有在休眠时持有互斥锁的锁，因为这会阻止另一个线程获取锁。锁会在超出范围时自动释放。
</div>

<div class="hint">
  好吧，实话实说，这对<i>我</i>来说也挺棘手的。
  我能看到你可能遇到的各种问题，所以现在我不确定你遇到的是哪个问题 :)
  请查看一些可能的<a href="https://github.com/carols10cents/rustlings/issues/3 ">答案</a> —— 我的稍微复杂一些，因为我决定在检查状态时希望看到当前完成的作业数量。

  如果您仍然遇到这些提示没有帮助的问题，或者查看了示例答案并不明白它们为什么有效而您的不有效，请打开一个问题。

  如果您从示例解决方案中学到了东西，建议您在几天后再回来尝试这个练习，以巩固所学内容 :)
</div>