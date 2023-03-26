import { Box, BottomNavigation, BottomNavigationAction } from "@mui/material";
import RestoreIcon from "@mui/icons-material/Restore";
import FavoriteIcon from "@mui/icons-material/Favorite";
import LocationOnIcon from "@mui/icons-material/LocationOn";
import { blue } from "@mui/material/colors";
import { useState } from "react";

export const NaviButtons = () => {
  const [value, setValue] = useState(0);
  return (
    <Box
      sx={{ 
        width: "100%",
        height: 60,
      }}
    >
      <BottomNavigation
        showLabels
        value={value}
        onChange={(event, newValue) => {
          setValue(newValue);
        }}
      >
        <BottomNavigationAction label="Main" icon={<RestoreIcon />} />
        <BottomNavigationAction label="Toplist" icon={<FavoriteIcon />} />
        <BottomNavigationAction label="Settings" icon={<LocationOnIcon />} />
      </BottomNavigation>
    </Box>
  );
};