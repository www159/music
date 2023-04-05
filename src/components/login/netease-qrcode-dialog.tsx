import { atomDialogOpen } from "@/stores/login";
import { atomStepIndexRead, atomStepNext, atomStepsRead } from "@/stores/netease-qrcode-dialog";
import { Box, Button, Dialog, DialogActions, DialogContent, DialogTitle, Stack, Step, StepLabel, Stepper } from "@mui/material";
import { QRCodeCanvas } from "qrcode.react";
import { useAtom } from "jotai";
import { useEffect, useState } from "react";
import { abortQrocdeSession, createQrcodeSession, getQrcode } from "@/services/invoke/rquest";
import { UnlistenFn } from "@tauri-apps/api/event";

export const NeteaseQrcodeDialog = () => {
  // SECTION state
  const [qrcode, setQrcode] = useState("");
  // ~SECTION
  // SECTION store
  const [dialogOpen, setDiagOpen] = useAtom(atomDialogOpen);
  const [steps] = useAtom(atomStepsRead);
  const [stepIndex] = useAtom(atomStepIndexRead);

  // ~SECTION

  // SECTION initialize effect
  useEffect(() => {
    getQrcode()
      .then(qrcode => {
        setQrcode(qrcode.rawData);
      });
  }, []);
  // ~SECTION
  return (
    <Dialog
      open={dialogOpen}
    >
      <DialogTitle>
          login
      </DialogTitle>
      <DialogContent 
        dividers
      >
        <Stack alignItems="center">
          <Box>
            <QRCodeCanvas value={qrcode} size={140} />
          </Box>
          <Stepper 
            nonLinear 
            activeStep={stepIndex}
            sx={{
              minWidth: {
                xs: "400px",
                md: "500px",
              },
              marginTop: "2ch"
            }}
          >
            {steps.map((step, index) => (
              <Step key={index}>
                <StepLabel>
                  {step}
                </StepLabel>
              </Step>
            ))}
          </Stepper>
        </Stack>
      </DialogContent>
      <DialogActions>
        <Button
          onClick={() => {
            setDiagOpen(false);
            abortQrocdeSession();
          }}
        >
            exit
        </Button>
      </DialogActions>
    </Dialog>
  );
};