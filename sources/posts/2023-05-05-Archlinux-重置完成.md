---
title: dwm 和 st 的安装及补丁
date: 2023-05-05 15:59:30
tags:
categories:
---

## 前言

得益于近些日子在学校深入学习了很多关于 Linux 的知识，加之之前也多次重装过 Arch ，这些因素综合起来后，这一次的系统安装可以说颇为顺畅，之前因无法理解的地方而导致难以下手的操作，所赖知识见长，逐渐克服且熟练了起来。

本次为了更加深入地去学习 Linux ，所以放弃了像 Gnome 这样简单易用的桌面环境, 而是使用像 DWM 这样更简单的窗口管理器，虽然是自找麻烦，但我也乐在其中，我会在本文中简略地记录下折腾的过程。

## DWM 和 st 的安装

事实上安装 DWM 非常的简单，自需要从 suckless 官网下载源码文件即可。
```bash

$ git clone git://git.suckless.org/dwm
$ cd dwm
$ make
# sudo make install

```

编译安装后，我这里是通过 xrog 的 `.xinitrc` 从 tty 终端使用 `startx` 命令启动的：
```bash
exec dwm
```

启动之后的第一眼就是简陋，目前还什么都干不了，因为我甚至还没安装终端，suckless 也提供了一个 xrog 下的终端实现，叫 `st` 。
通过魔法键回到 tty （ps.当然也可以通过快捷键：`Alt`  `+` `Shift` `+` `Q` 退出 dwm )，通过 tty 的终端去下载 st 源码。

同样也是通过源码安装：
```bash
$ git clone https://git.suckless.org/st
$ cd st
$ make
# sudo make install
```
编译安装完后，在dwm便应该通过`Alt` `+` `Shift` `+` `Enter` 组合键启动 `st` 管终端。但是毫无疑问，也是十分简陋，甚至字体小到无法看清……


不过问题不大，我学过 C ，源码文件是能看懂的，首先就是修改了字体大小，顺带一提我还通过 pacman 包管理下载了 [ttf-fira-code] 字体。

首先来修改一下 dwm 的字体和其大小：

```c
# dwm > config.h

static const char *fonts[]          = { "Fira Code:size=14" };
static const char dmenufont[]       = "Fira Code:size=14";

```
保存后通过 `sudo make clean install` 命令安装即可

接着修改 st 终端的字体和大小：

```c
# st > config.h

static char *font = "Fira Code:style=Bold:pixelsize=23:antialias=true:autohint=true";
```

保存后通过 `sudo make clean install` 命令安装即可

这样修改完后，便好看很多了。

## 安装 dmenu 

这个就是 suckless 的程序托盘？
总之，安装后，可以快速打开程序，也是蛮好的，这个就没有使用源码安装了，不过听说它也有很多好玩之处，之后再研究吧，总之 dmenu 通过包管理器安装：

```bash
$ sudo pacman -S dmenu
```

## 给 dwm 打补丁
Dwm 目前其实足够我使用了， 这里就打一个透明补丁: 「 [alpha.diff](https://dwm.suckless.org/patches/alpha/dwm-alpha-20201019-61bb8b2.diff
)  」

```bash
$ wget https://dwm.suckless.org/patches/alpha/dwm-alpha-20201019-61bb8b2.diff
$ patch p1 < dwm-alpha-20201019-61bb8b2.diff
 
```
通过 `patch` 命令 把补丁打进去即可，之前我都没怎么用过这个 patch 命令，用多了其实也还好，出现错误了，手动去修改源文件即可。

## 给 st 打补丁
这里打了好几个补丁：
- 「 [st-alpha-20220206-0.8.5.diff](https://st.suckless.org/patches/alpha/st-alpha-20220206-0.8.5.diff)  」 这是改变st透明度的补丁。
- 「 [st-scrollback-20210507-4536f46.diff](https://st.suckless.org/patches/scrollback/st-scrollback-20210507-4536f46.diff) 」这是让 st 终端支持回滚（按键支持）
- 「 [st-scrollback-mouse-20220127-2c5edf2.diff](https://st.suckless.org/patches/scrollback/st-scrollback-mouse-20220127-2c5edf2.diff) 」这是回滚的鼠标滚轮支持(ps.需要先安装scrollback 补丁)

也是一样地通过 `patch` 命令打进去，这里就不再赘述了。

## 安装 picom 
这是一个合成管理器，可以给窗口管理器带来需要效果，比如透明、阴影，上面的透明补丁也需要它的安装才能真正起作用。

```bash
$ sudo pacman -S picom
```

安装完后，编辑 ` ~/.xinitrc ` 文件,进程让它以 Daemon（后台） 的形式运行。
```bash
compton -b
```

（未完待续）

