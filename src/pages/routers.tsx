import React from "react";
import { MainPage } from "./main";

interface RouterListItem {
    label: string,
    link: string,
    element: () => JSX.Element,
}

export const routers: RouterListItem[] = [
  {
    label: "Label-Main",
    link: "/",
    element: MainPage,
  },
];