# Rust Tutorial. 《跟老卫学Rust开发》

![](images/rust-logo-blk.svg)

*Rust Tutorial*, is a book about how to develop Rust applications.



《跟老卫学Rust开发》是一本 Rust 应用开发的开源学习教程，主要介绍如何从0开始开发 Rust 应用。本书包括最新版本  Rust 1.79.0 中的新特性。图文并茂，并通过大量实例带你走近 Rust 的世界！

本书业余时间所著，由于笔者能力有限、时间仓促，书中难免有疏漏之处，欢迎读者批评指正。


## Samples 示例


* [hello_world](samples/hello_world)：Hello World
* [variable_demo](samples/variable_demo)：变量与常量
* [base_type_demo](samples/base_type_demo)：基本类型
* [function_demo](samples/function_demo)：函数
* [comment_demo](samples/comment_demo)：注释
* [flow_control_demo](samples/flow_control_demo)：流程控制
* [guessing_game](samples/guessing_game)：猜数字游戏
* [count_the_number_of_characters](samples/count_the_number_of_characters)：统计字符串的字符数
* [ownership_demo](samples/ownership_demo)：所有权概述
* [reference_demo](samples/reference_demo)：引用与借用
* [slice_demo](samples/slice_demo)：切片类型
* [convert_sentences_into_words](samples/convert_sentences_into_words)：将句子转为单词
* [convert_sentences_containing_period_into_words](samples/convert_sentences_containing_period_into_words)：将包含句号的句子转为单词
* [string_demo](samples/string_demo)：字符串
* [operation_string_demo](samples/operation_string_demo)：操作字符串
* [tuple_and_struct_demo](samples/tuple_and_struct_demo)：元组与结构体
* [enum_demo](samples/enum_demo)：枚举
* [array_demo](samples/array_demo)：数组
* [string_concatenation](samples/string_concatenation)：将单词转为句子
* [match_and_if_let_demo](samples/match_and_if_let_demo)：match和if let
* [option_match_demo](samples/option_match_demo)：在match中使用Option枚举类
* [other_pattern_demo](samples/other_pattern_demo)：其他模式适用场景
* [pattern_match_guessing_game](samples/pattern_match_guessing_game)：猜数字游戏
* [rock_paper_scissors](samples/rock_paper_scissors)：“剪刀石头布”游戏
* [generics_demo](samples/generics_demo)：泛型
* [trait_demo](samples/trait_demo)：特征
* [trait_object_demo](samples/trait_object_demo)：特征对象
* [universal_min_function](samples/universal_min_function)：通用min函数
* [universal_max_function](samples/universal_max_function)：通用max函数
* [vector_demo](samples/vector_demo)：动态数组
* [hashmap_demo](samples/hashmap_demo)：HashMap
* [hashset_demo](samples/hashset_demo)：HashSet
* [valid_bracket_sequence](samples/valid_bracket_sequence)：有效括号序列
* [min_stack](samples/min_stack)：最小栈
* [panic_demo](samples/panic_demo)： `panic!`处理不可恢复的错误
* [result_demo](samples/result_demo)：Result处理可恢复的错误
* [unwrap_expect_demo](samples/unwrap_expect_demo)：简化错误处理
* [get_file_metadata](samples/get_file_metadata)：获取文件类型及大小
* [hello_cargo](samples/hello_cargo)：crate和包
* [backyard](samples/backyard)：模块
* [restaurant](samples/restaurant)：使用路径引用模块
* [restaurant_separating_modules](samples/restaurant_separating_modules)：将模块拆分成多个文件
* [nation_separating_modules](samples/nation_separating_modules)：将行政机构拆分到不同的模块
* [println_format_demo](samples/println_format_demo)：格式化输出概述
* [formatted_output](samples/formatted_output)：输出大X图形
* [lifetime_demo](samples/lifetime_demo)：悬垂引用和生命周期
* [lifetime_function_demo](samples/lifetime_function_demo)：函数中的生命周期
* [lifetime_annotations](samples/lifetime_annotations)：添加生命周期注解
* [iterator_demo](samples/iterator_demo)：迭代器
* [iterator_test_demo](samples/iterator_test_demo)：for循环与迭代器的性能比较
* [count_the_total_number_of_rabbits_each_month](samples/count_the_total_number_of_rabbits_each_month)：统计每个月兔子的总数
* [type_converse_demo](samples/type_converse_demo)：类型转换
* [newtype_demo](samples/newtype_demo)：类型别名和`newtype`模式
* [type_conversion_to_string](samples/type_conversion_to_string)：将任何类型转换成String
* [box_demo](samples/box_demo)：Box堆对象分配
* [deref_demo](samples/deref_demo)：Deref解引用
* [drop_demo](samples/drop_demo)：Drop释放资源
* [rc_demo](samples/rc_demo)：Rc引用计数智能指针
* [cell_demo](samples/cell_demo)：Cell与内部可变性模式
* [weak_demo](samples/weak_demo)：一个循环引用的例子
* [weak_solve_circular_reference_demo](samples/weak_solve_circular_reference_demo)：使用Weak解决循环引用
* [bank_balance](samples/bank_balance)：银行存取款
* [thread_spawn_demo](samples/thread_spawn_demo)：创建新线程
* [thread_join_demo](samples/thread_join_demo)：等待子线程结束
* [thread_move_demo](samples/thread_move_demo)：将move闭包与线程一同使用
* [thread_barrier_demo](samples/thread_barrier_demo)：使用线程屏障Barrier
* [channel_single_producer_demo](samples/channel_single_producer_demo)：单发送者单接收者
* [channel_try_recv_demo](samples/channel_try_recv_demo)：非阻塞接收消息
* [channel_for_demo](samples/channel_for_demo)：接收多条消息
* [channel_multiple_producer_demo](samples/channel_multiple_producer_demo)：多发送者单接收者
* [channel_sync_demo](samples/channel_sync_demo)：使用同步通道
* [mutex_multi_thread_demo](samples/mutex_multi_thread_demo)：多线程中使用互斥器
* [rwlock_demo](samples/rwlock_demo)：使用读写锁
* [condvar_demo](samples/condvar_demo)：使用条件变量
* [atomic_demo](samples/atomic_demo)：使用原子类型
* [dining_philosophers](samples/dining_philosophers)：哲学家就餐问题
* [global_variable_const_demo](samples/global_variable_const_demo)：使用静态常量
* [global_variable_static_demo](samples/global_variable_static_demo)：使用静态变量
* [global_variable_atomic_demo](samples/global_variable_atomic_demo)：使用原子类型
* [global_variable_lazy_static_demo](samples/global_variable_lazy_static_demo)：使用`lazy_static!`
* [global_variable_box_leak_demo](samples/global_variable_box_leak_demo)：使用`Box::leak`
* [global_id_generator](samples/global_id_generator)：全局ID生成器
* [error_combinator_demo](samples/error_combinator_demo)：组合器
* [error_customize_demo](samples/error_customize_demo)：自定义错误类型
* [error_customize_for_get_file](samples/error_customize_for_get_file)：自定义获取不到文件时的错误类型
* [unsafe_fn_demo](samples/unsafe_fn_demo)：调用不安全函数或方法
* [unsafe_union_demo](samples/unsafe_union_demo)：访问联合体中的字段
* [unsafe_stack](samples/unsafe_stack)：实现简单的栈数据结构
* [macro_rules_demo](samples/macro_rules_demo)：声明式宏
* [custom_derive_demo](samples/custom_derive_demo)：自定义derive过程宏
* [attribute_macro_demo](samples/attribute_macro_demo)：属性宏
* [function_like_macro_demo](samples/function_like_macro_demo)：类函数宏
* [print_message_function_like_macro](samples/print_message_function_like_macro)：打印信息的宏
* [async_await_demo](samples/async_await_demo)：`async/await`简单入门
* [multi_futures_join_demo](samples/multi_futures_join_demo)：使用`join!`
* [multi_futures_select_demo](samples/multi_futures_select_demo)：使用`select!`
* [singing_and_dancing](samples/singing_and_dancing)：载歌载舞
* [implementing_polymorphism_using_trait_demo](samples/implementing_polymorphism_using_trait_demo)：使用特征对象实现多态性
* [oo_design_patterns_demo](samples/oo_design_patterns_demo)：面向对象设计模式的实现
* [oo_for_shape](samples/oo_for_shape)：面向对象的图形
* [test_lib_demo](samples/test_lib_demo)：编写测试及控制执行
* [performance_test_of_accumulator](samples/performance_test_of_accumulator)：累加器的性能测试
* [reqwest_http_client](samples/reqwest_http_client)：使用HTTP请求JSON数据
* [env_logger_demo](samples/env_logger_demo)：使用具体的日志库
* [tracing_demo](samples/tracing_demo)：使用tracing记录日志
* [simple_logger](samples/simple_logger)：实现日志框架
* [single_thread_web_server](samples/single_thread_web_server)：实现Web服务器
* [kv_store](samples/kv_store)：实现KV数据库
* 未完待续...



## Get start 如何开始阅读

选择下面入口之一：

* <https://github.com/waylau/rust-tutorial> 的 [README.md](https://github.com/waylau/rust-tutorial/blob/master/README.md) 


## Code 源码

书中所有示例源码，移步至<https://github.com/waylau/rust-tutorial>的 `samples` 目录下。



## Issue 意见、建议

如有勘误、意见或建议欢迎拍砖 <https://github.com/waylau/rust-tutorial/issues>

## Contact 联系作者

* Blog: [waylau.com](http://waylau.com)
* Gmail: [waylau521(at)gmail.com](mailto:waylau521@gmail.com)
* Weibo: [waylau521](http://weibo.com/waylau521)
* Twitter: [waylau521](https://twitter.com/waylau521)
* Github : [waylau](https://github.com/waylau)

## Support Me 请老卫喝一杯

![开源捐赠](https://waylau.com/images/showmethemoney-sm.jpg)
