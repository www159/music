import { Box, Typography } from "@mui/material";

export const PlayerBar = () => {
  return (
    <Box
      sx={{
        width: "100%",
        height: "50px",
        bgcolor: "secondary.main",
        position: "fixed",
        bottom: "0",
      }}>
      <Typography>
        this is player bar!
      </Typography>
    </Box>
  );
};