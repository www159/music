// import { themeSwitchMachine } from "@/machines/themeSwitch";
import useSWR from "swr";
// import { useMachine } from "@xstate/react";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { getBaseConfig } from "@/services/invoke/config";
import { BaseErrorBoundary } from "@/components/base/base-error-bountery";
import { routers } from "./routers";
import { Route, Routes } from "react-router-dom";
import { Box, Divider, Grid, Paper, Stack, ThemeProvider, createTheme } from "@mui/material";
import { PlatformBar } from "@/components/layout/platform-bar";
import { NaviButtons } from "@/components/layout/navi-buttons";
import { PlayerBar } from "@/components/layout/player-bar";


const theme = createTheme({
  palette: {
    mode: "light",
  },
});

export const Layout = () => {
  //SECTION store
  //i18n
  const { t } = useTranslation();
  // theme: light | dark
  const { data: baseConfig } = useSWR(
    "getBaseConfig",
    getBaseConfig
  );

  const { theme_mode } = baseConfig ?? {};
  //~SECTION

  

  //SECTION - initialize
  useEffect(() => {
    // get theme from backend
    // TODO use material default theme, customize theme

    return undefined;
  }, []);
  //~SECTION

  return (
    <BaseErrorBoundary>
      <ThemeProvider theme={
        theme
      }>
        <Paper
          sx={{
            display: "flex",
            alignItems: "center",
            height: "100vh",
            width: "100vw",
          }}
        >
          <Stack
            direction={"row"}
            divider={<Divider orientation="vertical" flexItem />}
            spacing={1}
            sx={{
              height: "98%",
              width: "100%",
            }} 
          > 
            <PlatformBar />
            <Stack
              justifyContent={"space-between"}
              alignItems={"streth"}
              sx={{
                width: "100%",
                height: "100%"
              }}>
              <NaviButtons />
              <Routes>
                {routers.map(({ label, link, element: Elm }) => (
                  <Route key={label} path={link} element={<Elm />} />
                ))}
              </Routes>
              <PlayerBar />
            </Stack>
          </Stack>
        </Paper>
      </ThemeProvider>
    </BaseErrorBoundary>
  );
};