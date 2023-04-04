import { MainPage } from "./main";
import { LoginPage } from "./login";

interface RouterListItem {
    label: string,
    link: string,
    element: () => JSX.Element,
}

export const layoutRouters: RouterListItem[] = [
  {
    label: "label-main",
    link: "/",
    element: MainPage,
  },
  {
    label: "label-login",
    link: "/login",
    element: LoginPage,
  }
];