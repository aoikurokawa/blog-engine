#!/usr/bin/env bash
default_num=8192
ulimit_num= ulimit -n

#limits for open file
if ((ulimit_num < default_num))
then
  ulimit -n 8192
fi

cargo test
