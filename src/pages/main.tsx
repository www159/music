import { RecommandList } from "@/components/main/recommand-list";
import { SearchBar } from "@/components/main/search-bar";
import { Box, Stack } from "@mui/material";

export const MainPage = () => {
  return (

    <Stack
      spacing={2}
      sx={{
        width: "100%",
        height: "100%",
        overflowY: "scroll",
      }}
    >
      <SearchBar />
      <RecommandList label={"hotest"} />
      <RecommandList label={"customize"} />
    </Stack>

  );
};