import { RecommandList } from "@/components/main/recommand-list";
import { SearchBar } from "@/components/main/search-bar";
import { Box, Stack, Typography } from "@mui/material";

export const MainPage = () => {
  return (
    <Box 
      sx={{
        height: "100%",
        width: "100%",
      }}>
      <Stack
        spacing={2}
      >
        <SearchBar />
        <RecommandList label={"hotest"} />
        <RecommandList label={"customize"} />
      </Stack>
    </Box>
  );
};