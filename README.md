# Tauri + Vanilla

https://github.com/tauri-apps/tauri/issues/7957

This repo is a small reproduction of the dynamic tray menu. Each second we
replace the menu with a new one. You can check this by opening the menu multiple
times. However, when the menu is opened you will not see any updates until you
reopen the menu.

## Installing

```
git clone git@github.com:vovayatsyuk/tauri-dynamic-tray-menu.git &&\
cd tauri-dynamic-tray-menu &&\
npm ci
```

## Running

```
npm run tauri dev
```
