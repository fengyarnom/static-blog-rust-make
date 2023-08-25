---
title: '[NFS] mount: Connection timed out'
date: 2023-04-06 20:09:29
tags:
- NFS
- ARM
categories:
- ARM 学习总结
---

## 问题描述
Linux 服务端和 Arm 开发板客户端进行 NFS 服务的连接。

Linux 和 Arm 开发板之间能ping通，并且处于同一网端，且掩码、网口相同，但是在执行下面的程序时发生超时错误。

在服务端的nfs配置完全正确的情况下，Arm开发板还是无法正确连接nfs服务器。

```bash
[root@FORLINX6410]# mount -t nfs -o nolock,hard 192.168.1.5:/home/yarnom/nfs /mnt                                                                               
mount: mounting 192.168.1.5:/home/yarnom/nfs on /mnt failed: Connection timed out

```

## 解决
这个问题困扰了我两天终于在这个[帖子](https://stackoverflow.com/questions/45938202/mount-nfs-connection-timed-out-on-ubuntu-14-04-1-lts)里找到了解决方案。
> Mount the NFS filesystem using the TCP protocol instead of the default UDP protocol. Many NFS servers only support UDP.

这是在NFSv3中添加了对TCP协议的支持：

总之，我尝试了下面的命令，使用了tcp协议：
```bash
$ mount -t nfs -o nolock,proto=tcp,port=2049 192.168.1.5:/home/yarnom/nfs /mnt
```

这个协议最终使我正确连接上了nfs服务器。

