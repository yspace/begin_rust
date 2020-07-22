
## 运行测试;

~~~cmd

    cargo test
~~~

只运行集成测试:
> cargo test --test integration_test

子目录下的模块
> Files in subdirectories of the tests directory don’t get compiled as separate crates 
or have sections in the test output.