import { Box, Button, Divider, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Tooltip, Typography } from "@mui/material";
import MoreHorizIcon from "@mui/icons-material/MoreHoriz";
import { useAtom } from "jotai";
import { useEffect } from "react";
import { Link } from "react-router-dom";
import { atomPlatforms } from "@/stores/login";

export const PlatformBar = () => {

  // SECTION store
  const [platforms] = useAtom(atomPlatforms);
  // ~SECTION
  
  // SECTION initialize effect
  useEffect(() => {
    // TODO
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
                <ListItemButton component={Link} to={"/"}>
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
            <ListItemButton component={Link} to="/login">
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

