#!/bin/sh

DEST='/usr/local'
ROOT=`dirname $0`

cp -v $ROOT/target/release/illuminanced $DEST/sbin/ || exit 1
cp -v $ROOT/illuminanced.toml $DEST/etc || exit 1
cp -v illuminanced.service /etc/systemd/system/illuminanced.service || exit 1
systemctl enable illuminanced.service || exit 1
systemctl start illuminanced.service || exit 1
systemctl status illuminanced.service || exit 1
