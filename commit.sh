#!/bin/bash 
comments=$@
echo '*' `date +"%a %D/%T":` $comments | tee -a c.log 
readme='/dev/null'

echo '
README
============================== 
Leetcode(www.leetcode.com) solutions with Rust programming language. 

***** 
|Author|@117|
|:---  |:---
|E-mail|ssruoz@gmail.com

### Log: 
```' > $readme

cat c.log >> $readme
echo '```' >> $readme 

# auto update README.md
python3 scripts/auto_add_log.py 

git add .
git commit -m "$comments"
git push 