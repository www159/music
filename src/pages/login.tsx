import { NeteaseQrcodeDialog } from "@/components/login/netease-qrcode-dialog";
import { createQrcodeSession } from "@/services/invoke/rquest";
import { atomDialogOpen } from "@/stores/login";
import { atomStepNext } from "@/stores/netease-qrcode-dialog";
import { Box, Button, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, List, ListItem, ListItemButton, ListItemText, ListSubheader, Paper, Typography } from "@mui/material";
import { useAtom } from "jotai";
import { useEffect } from "react";

export const LoginPage = () => {
  // SECTION store
  const [dialogOpen, setDiagOpen] = useAtom(atomDialogOpen);
  const [, stepNext] = useAtom(atomStepNext);
  // ~SECTION

  // SECTION initialize effect
  useEffect(() => {
    //TODO
  }, []);
  // ~SECTION
  return (
    <Box
      sx={{
        width: "100%",
        height: "100%",
        bgcolor: "primary.main",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Paper
        sx={{
          width: "200px",
        }}
      >
        <List>
          <ListSubheader>
          platform
          </ListSubheader>
          <ListItem>
            <ListItemButton
              onClick={() => { 
                setDiagOpen(true);
                createQrcodeSession((val) => {
                  switch(val) {
                  case 1: stepNext(); break;
                  default: break;
                  }
                }); 
              }}
            >
              <ListItemText primary={<Typography>
              Netease
              </Typography>} 
              />
            </ListItemButton>
          </ListItem>
          <ListItem>
            <ListItemButton>
              <ListItemText primary={<Typography>
              other
              </Typography>} 
              />
            </ListItemButton>
          </ListItem>
        </List>
      </Paper>
      <NeteaseQrcodeDialog />
    </Box>
  );
};