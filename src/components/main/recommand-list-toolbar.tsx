import { Typography, Box, IconButton, debounce } from "@mui/material";
import { MoreHoriz } from "@mui/icons-material";
import { testCookie, testCookieLoad } from "@/services/invoke/rquest";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

// SECTION props type
interface RecommandListToolBarProps {
  label: string
}
// ~SECTION

export const RecommandListToolbar = (props: RecommandListToolBarProps) => {
  // SECTION props
  const { label } = props;
  // ~SECTION

  const [num, setNum] = useState(0);

  // SECTION initialize effect
  useEffect(() => {
    const unlisten = listen("test-emit", ({ payload }) => {
      setNum(num => num + 1);
      console.log(payload);

      return () => {
        unlisten.then(f => f());
      };
    });
  }, []);
  // ~SECTION
  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center"
      }}
    >
      <Typography
        sx={{
          fontSize: "small",
          marginRight: "1ch",
        }}
      >
        {label}
        {num}
      </Typography>
      <IconButton onClick={() => {
        // testCookie();
        testCookieLoad();
      }}>
        <MoreHoriz fontSize="small" />
      </IconButton>
    </Box>
  );
};