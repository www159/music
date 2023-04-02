import { Box, Divider, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Tooltip, Typography } from "@mui/material";
import MoreVertIcon from "@mui/icons-material/MoreVert";
import MoreHorizIcon from "@mui/icons-material/MoreHoriz";
import { useAtom } from "jotai";
import { atomPlatforms } from "@/stores/platform-bar";
import { useEffect } from "react";

export const PlatformBar = () => {

  // SECTION store
  const [platforms, setPlatforms] = useAtom(atomPlatforms);
  // ~SECTION
  
  // SECTION initialize effect
  useEffect(() => {
    setPlatforms([
      "Netease",
      "Blank",
    ]);
  }, []);
  return (
    <Box sx={{
      height: "100%",
      width: "70px",
    }}>
      <nav aria-label="main mailbox folders">
        <List>
          {platforms.map(platform => (
            <ListItem
              key={platform} 
              disablePadding 
              alignItems="center"
            >
              <Tooltip 
                title= {platform}
                placement="right"
              >
                <ListItemButton>
                  <ListItemText primary={
                    <Typography
                      sx={{
                        textOverflow: "clip",
                        overflow: "hidden",
                      }}
                    >
                      {platform.substring(0, 3)}..
                    </Typography>
                  } />
                </ListItemButton>
              </Tooltip>
            </ListItem>
          ))}
          <Divider />
          <ListItem disablePadding>
            <ListItemButton> 
              <ListItemIcon>
                <MoreHorizIcon />
              </ListItemIcon>
            </ListItemButton>
          </ListItem>
        </List>
      </nav>
    </Box>
  );
};

