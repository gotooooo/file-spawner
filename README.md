# file-spawner

## run

```shell
#cargo run {file_size} {quantity} {directory_name}
cargo run 1000000 10 tmp
```

```shell
$ ls -al tmp
.rw-r--r-- 1.0M gotooooo 27 3 16:33 2a25b2de677a44aba5dda6ddc68f67cd
.rw-r--r-- 1.0M gotooooo 27 3 16:33 2eb44ba6d82e42fd83dbe02b6e289406
.rw-r--r-- 1.0M gotooooo 27 3 16:33 3bfe7ee5b6f04cc0a39fd348ab84e52b
.rw-r--r-- 1.0M gotooooo 27 3 16:33 8ef810b1ec734825b2122ef240d60d63
.rw-r--r-- 1.0M gotooooo 27 3 16:33 21cfaa86494949bf8521448fb83f41e6
.rw-r--r-- 1.0M gotooooo 27 3 16:33 881082f3bce0486f95799edc2aa56d9e
.rw-r--r-- 1.0M gotooooo 27 3 16:33 b8beacc4be1b45ff9ef00f2635ef4546
.rw-r--r-- 1.0M gotooooo 27 3 16:33 c1e3cf6934ec45c78ad7f9f38395e237
.rw-r--r-- 1.0M gotooooo 27 3 16:33 e190741f62b14a1089c9ce69682c2f01
.rw-r--r-- 1.0M gotooooo 27 3 16:33 ec3458bba67a4d79ac5343064cb335d9
```

## verbose

This app use [clap-verbosity-flag](https://github.com/rust-cli/clap-verbosity-flag).

```shell
# show warn level logs
cargo run 1000000 10 tmp -v
# show info level logs
cargo run 1000000 10 tmp -vv
```

```shell
$ cargo run 10 10 tmp -vv
[2022-03-27T08:13:47Z WARN  file_spawner] tmp directory is AlreadyExists
[2022-03-27T08:13:47Z INFO  file_spawner] 1150fb111fcb4c6c9571d952e0ba24a8 created.
[2022-03-27T08:13:47Z INFO  file_spawner] 11e18271d24a49efa41e114d515643ac created.
[2022-03-27T08:13:47Z INFO  file_spawner] 75d2857c282d4d7fa4810e321ba9a059 created.
[2022-03-27T08:13:47Z INFO  file_spawner] 51df3f49f8934216b65b586b338370b6 created.
[2022-03-27T08:13:47Z INFO  file_spawner] c1a98bf730ca4de5bc6630c38ad2e153 created.
[2022-03-27T08:13:47Z INFO  file_spawner] 6cd1ec5e62b14354ab2b909e8e12a5e5 created.
[2022-03-27T08:13:47Z INFO  file_spawner] 02db1e3f769142a89015d1498fae32d7 created.
[2022-03-27T08:13:47Z INFO  file_spawner] 1e5c5858a54244719759afe51e7ad54f created.
[2022-03-27T08:13:47Z INFO  file_spawner] 0165464567df4aec8b01f903c2544dcd created.
[2022-03-27T08:13:47Z INFO  file_spawner] f2646f4d71774a33b2cbb717f50825ee created.
```
