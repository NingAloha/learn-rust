## 收集所有句柄

多线程应用程序的一个挑战是主线程可能会在创建的线程完成之前结束。`thread::spawn` 函数返回 `JoinHandle` 结构体。收集 JoinHandles 并等待它们完成。

<div class="hint">
将从 <code>thread::spawn</code> 函数获得的值推入 <code>handles</code> 向量中。
</div>

<div class="hint">
要加入线程，您需要在相应的 <code>JoinHandle</code> 上调用 <code>join</code> ，然后 <code>unwrap</code> 结果。
</div>