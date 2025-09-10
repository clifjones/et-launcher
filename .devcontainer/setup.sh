#/usr/bin/env bash

rustup update
npm install -g npm
#npm install -g @tauri-apps/cli@latest
npm install -g repomix

if [ ! -e et-os-addons ] ; then
  git clone --recursive https://github.com/clifjones/et-os-addons.git
fi
sudo cp -r et-os-addons/emcomm-tools-os-community/overlay/opt /
cp -r et-os-addons/emcomm-tools-os-community/overlay/etc/skel/. $HOME/.
