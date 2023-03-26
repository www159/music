import { Box, Typography } from "@mui/material";

export const PlayerBar = () => {
  return (
    <Box
      sx={{
        width: "100%",
        height: "100px",
        bgcolor: "secondary.main",
      }}>
      <Typography>
        this is player bar!
      </Typography>
    </Box>
  );
};