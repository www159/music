import { NeteaseQrcodeDialog } from "@/components/login/netease-qrcode-dialog";
import { createQrcodeSession, getUserAccount } from "@/services/invoke/rquest";
import { atomDialogOpen, atomLoginedPlatforms, atomOpenedDialog, atomOpenedDialogIndex, atomPlatforms, atomSetLogin } from "@/stores/login";
import { atomStepNext } from "@/stores/netease-qrcode-dialog";
import { CheckCircle, Login } from "@mui/icons-material";
import { Box, Button, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, List, ListItem, ListItemButton, ListItemIcon, ListItemText, ListSubheader, Paper, Typography } from "@mui/material";
import { useAtom } from "jotai";
import { useEffect } from "react";

export const LoginPage = () => {
  // SECTION store
  const [dialogOpen, setDialogOpen] = useAtom(atomDialogOpen);
  const [OpenedDialog] = useAtom(atomOpenedDialog);
  const [, setOpenedDialogIndex] = useAtom(atomOpenedDialogIndex);
  const [platforms] = useAtom(atomPlatforms);
  const [loginedPlatforms] = useAtom(atomLoginedPlatforms);
  const [, setLogin] = useAtom(atomSetLogin);
  // ~SECTION

  // SECTION initialize effect
  useEffect(() => {
    console.log("logined: ", loginedPlatforms);
    // checklogin
    getUserAccount()
      .then((data) => {

        console.log("recieve!");
        if(data.userId !== 0) {
          setLogin(["Netease", true]);
          console.log(loginedPlatforms);
        }
      });
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
          {platforms.map((platform, index) => (
            <ListItem key={index}>
              <ListItemButton
                onClick={() => { 
                  if(loginedPlatforms[index]) {
                    // TODO
                  }
                  else {
                    setOpenedDialogIndex(index);
                    setDialogOpen(true);
  
                  }
                  // setDiagOpen(true);
                  // createQrcodeSession((val) => {
                  //   switch(val) {
                  //   case 1: stepNext(); break;
                  //   default: break;
                  //   }
                  // }); 
                }}
                sx={{
                  display: "flex",
                  justifyContent: "space-between",
                }}
              >
                <ListItemText primary={<Typography>
                  {platform}
                </Typography>} 
                />
                {loginedPlatforms[index] && <ListItemIcon
                  sx={{
                    color: "green",
                    justifyContent: "flex-end",
                  }}
                >
                  <CheckCircle />
                </ListItemIcon>}
              </ListItemButton>
            </ListItem>
          ))}
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
      <Dialog 
        open={dialogOpen}
      >
        <OpenedDialog />
      </Dialog>
      {/* <NeteaseQrcodeDialog /> */}
    </Box>
  );
};