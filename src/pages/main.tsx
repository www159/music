import { NaviButtons } from "@/components/layout/navi-buttons";
import { RecommandList } from "@/components/main/recommand-list";
import { SearchBar } from "@/components/main/search-bar";
import { Box, Stack } from "@mui/material";

// ANCHOR - platform spec page 
export const MainPage = () => {
  return (

    <Stack
      spacing={2}
      sx={{
        width: "100%",
        height: "calc(100% - 50px)",
        overflowY: "scroll",
      }}
    >
      <NaviButtons />
      <SearchBar />
      <RecommandList label={"hotest"} />
      <RecommandList label={"customize"} />
    </Stack>

  );
};