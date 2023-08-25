---
title: Extract tar.gz file to a directory and create the directory if not exist
date: 2023-05-09 16:14:24
tags:
categories:
---

## Description

I can run the following command to extract the tar.gz file to 'somedir' directory, But this command requires that the directory already exists.

```bash
tar -xvf somefile.tar.gz -C somedir
```


## Solution
Following command can extract the somefile.tar.gz file to a directory that does not exist.

```bash
tar -xf somefile.tar.gz --one-top-level=somedir
```

