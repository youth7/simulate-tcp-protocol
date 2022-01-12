
# 关于`syn`和`ack`
## 发送`syn`包的时候，`ack`需要怎么设置？
RFC中似乎没有给出描述，抓包结果来看是设置为0。从语义上来说此时`ack`值为多少都没有关系，应该被接收端忽略；**只有经过第二步的握手之后，对方传来了它的`seq`**，此时才能知道根据这个`seq`去设置自己的`ack`。  
可以再一次体现出三次握手的重点：**双方交换初始化信息**


## `ack`、`syn`是否消耗`seq`
`ack`不消耗，`syn`消耗。

## 状态机中一些出乎意料的细节
* 《TCP/IP详解 卷1：协议》中提及>
    > The state transition from LISTEN to SYN_SENT is legal but is not supported in Berkeley-derived implementations  

    这个说法非常意外，还没有从RFC793上找到相关描述，先暂时忽略。
    






# 参考
https://harttle.land/2014/09/27/tcp.html#header-11