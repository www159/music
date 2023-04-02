import { Typography, Box, IconButton } from "@mui/material";
import { MoreHoriz } from "@mui/icons-material";
import { testCookie, testCookieLoad } from "@/services/invoke/rquest";

// SECTION props type
interface RecommandListToolBarProps {
  label: string
}
// ~SECTION

export const RecommandListToolbar = (props: RecommandListToolBarProps) => {
  // SECTION props
  const { label } = props;
  // ~SECTION

  // SECTION hook function

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