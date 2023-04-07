import { LoginDialog } from "@/components/login/login-dialog";
import { NeteaseQrcodeDialog } from "@/components/login/netease-qrcode-dialog";
import { atom } from "jotai";
import React from "react";


export const atomPlatforms = atom<string[]>([
  "Netease",
  "QQ",
  "Blank",
]);


export const atomLoginedPlatforms = atom<boolean[]>([]);

export const atomSetLogin = atom(
  null,
  (get, set, [platform, login]: [string, boolean]) => {
    const loginedPlatforms = get(atomLoginedPlatforms);
    const platforms = get(atomPlatforms);
    const index = platforms.indexOf(platform);
    loginedPlatforms[index] = login;
    
    set(atomLoginedPlatforms, [...loginedPlatforms]);
  }
);
  
const atomLoginDialogs = atom<React.FC[]>([
  NeteaseQrcodeDialog,
  LoginDialog,
  LoginDialog,
]);

export const atomDialogOpen = atom(false);
  
export const atomOpenedDialogIndex = atom<number>(0);
  
export const atomOpenedDialog = atom(get => {
  const loginDialogs = get(atomLoginDialogs);
  const openedDialogIndex = get(atomOpenedDialogIndex);
  return loginDialogs[openedDialogIndex];
});

