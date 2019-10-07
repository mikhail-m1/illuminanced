#!/bin/sh

DEST='/usr/local'
ROOT=`dirname $0`

if [ -e /etc/systemd/system/illuminanced.service ]; then
    echo -n "Try stop service: "
    systemctl stop illuminanced.service || echo " Failed"
    echo " Done"
fi

if [ ! -d $DEST/etc ]; then
    mkdir $DEST/etc || exit "Cannot create $/DEST/etc"
fi

if [ ! -d $DEST/sbin ]; then
    mkdir $DEST/sbin || exit "Cannot create $/DEST/sbin"
fi

if [ -e $DEST/etc/illuminanced.toml ]; then
    echo "\nBackup old config file:"
    mv -v $DEST/etc/illuminanced.toml $DEST/etc/illuminanced.toml_`date +%y%m%d_%H%M%S`;
fi

echo "\nInstall:"

cp -v $ROOT/target/release/illuminanced $DEST/sbin/ || exit 1
cp -v $ROOT/illuminanced.toml $DEST/etc || exit 1
cp -v illuminanced.service /etc/systemd/system/illuminanced.service || exit 1

echo "\nStart service:"

systemctl enable illuminanced.service || exit 1
systemctl start illuminanced.service || exit 1
systemctl status illuminanced.service || exit 1
