# rust 重写rocketmq

## 1. rocketmq架构

- nameserver
- broker


broker 注册到nameserver
- 1、broker_bootstrap.rs:boot line: 30
- 2、broker_runtime.rs:start line: 653
- 3、broker_runtime.rs:register_broker_all line: 761
- 4、broker_runtime.rs:do_register_broker_all line: 857
- 5、out_api/broker_outer_api.rs:register_broker_all line: 128
- 6、out_api/broker_outer_api.rs:register_broker line: 203
- 7、clients/rocketmq_default_impl.rs:invoke_async line: 324




## 2. rocketmq的使用

发送消息到nameserver

- 1、bin/namesrv_bootstrap_server.rs:boot line: 49
- 2、runtime/server.rs:server.run(request_processor).await line:71
- 3、runtime/server.rs:listener.run() line:377
- 4、runtime/server.rs:handler.handle().await line: 106
- 5、runtime/server.rs:self.request_processor.process_request line: 138
- 6、processor.rs:process_request line: 49
- 7、processor/client_request_processor.rs:process_request line 118 或者 processor/default_request_processor.rs:process_request line 70
- 


broker接收消息

- 1、broker_bootstrap.rs::broker_runtime.start() line 43
- 2、broker_runtime.rs::start_basic_service() line 615
- 3、broker_runtime.rs::server.run(request_processor).await line: 626 或者 broker_runtime.rs::fast_server.run(fast_request_processor).await line 631


https://juejin.cn/column/7188043825917460535

### 一些类

RemotingCommand 是 RocketMQ 请求、响应的对象载体，可以理解成是 RocketMQ 定义的网络通信协议。在发送请求时，会根据当前请求和数据构建一个 RemotingCommand 对象，然后经过序列化、编码成字节码，再经过 Netty 的 Channel 发送出去，另一端则从 Channel 读取字节码，经过解码、反序列化成 RemotingCommand。
RemotingCommand 主要包含如下属性：

- code：RocketMQ 中每个请求都会有一个对应的编码，请求码的枚举定义在 RequestCode 中。
- opaque：每个请求的ID，通过内存的一个原子计数器 requestId 自增。
- flag：类型标识，有 请求、响应、Oneway 三种类型
- extFields：扩展字段
- body：请求主体数据的字节数组




