import { Typography, Box } from "@mui/material";
import { MoreHoriz } from "@mui/icons-material";

interface RecommandListToolBarProps {
  label: string
}

export const RecommandListToolbar = (props: RecommandListToolBarProps) => {
  const { label } = props;
  return (
    <Box
      sx={{
        display: "flex",
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
      <MoreHoriz fontSize="small" />
    </Box>
  );
};