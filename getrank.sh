
# echo -n "Current rank:  "
# /usr/local/bin/proxychains4 -q curl -s 'https://leetcode.com/fire4fly/' | grep 100000 | awk '{print $2}' | perl -ne 'print $1, "\n" if /(\d+)/'

# echo -n "Progress:      "
# /usr/local/bin/proxychains4 -q curl -s 'https://leetcode.com/fire4fly/' | grep " / " | sed -n 2,2p | awk '{print $1,"/",$NF}'

curl --socks5 127.0.0.1:1080 'bwg.fayderlau.xyz:81/rank.txt'
