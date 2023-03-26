import { Box, Divider, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Typography } from "@mui/material";

export const PlatformBar = () => {
  return (
    <Box sx={{
      height: "100%",
      width: "60px",
    }}>
      <nav aria-label="main mailbox folders">
        <List
          sx={{
            bgcolor: ""
          }}
        >
          <ListItem disablePadding alignItems="center">
            <ListItemButton>
  
              <ListItemText primary={
                <Typography>
                  Netease
                </Typography>
              } />
            </ListItemButton>
          </ListItem>
          <ListItem disablePadding>
            <ListItemButton >
              <ListItemText primary="Else" />
            </ListItemButton>
          </ListItem>
          <Divider />
          <ListItem disablePadding>
            <ListItemButton >
              <ListItemText primary="Add" />
            </ListItemButton>
          </ListItem>
        </List>
      </nav>
    </Box>
  );
};