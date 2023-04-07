import { listPlaylist } from "@/services/invoke/rquest";
import { atomPlaylistId, atomPlaylists, atomPlaylistsSome } from "@/stores/recommend-list";
import { Avatar, Box, Button, Grid, Paper, Stack, Typography, debounce } from "@mui/material";
import { useAtom } from "jotai";
import { useEffect } from "react";
import { RecommandListToolbar } from "./recommand-list-toolbar";
import { useNavigate } from "react-router-dom";

// SECTION props type
interface RecommandListProps {
  label: string
}
// ~SECTION
export const RecommandList = (props: RecommandListProps) => {
  // SECTION props
  const { label } = props;
  // ~SECTION

  // SECTION store
  const [, setPlaylists] = useAtom(atomPlaylists);
  const [playlists] = useAtom(atomPlaylistsSome);
  const [, setPlaylistId] = useAtom(atomPlaylistId);
  const navigate = useNavigate();
  // ~SECTION
  
  // SECTION initialize effect
  useEffect(() => {
    listPlaylist({})
      .then((playlists) => {
        setPlaylists(playlists);
      });
  }, []);
  // ~SECTION
  return (
    <Stack
      sx={{
        // height: "1000px",
      }}
    >
      <RecommandListToolbar label={label} />
      <Paper 
        elevation={3}
        sx={{
          marginRight: "5px",
        }}
      >
        <Grid
          container
          columns={{ xs: 2, sm: 4, md: 12 }}
          sx={{
            marginTop: "6px",
          }}
        >
          {playlists.map(playlist => (
            <Grid 
              item
              xs={1}
              md={2} 
              key={playlist.id}
            >
              <Button
                sx={{
                  display: "flex",
                  alignItems: "center",
                  flexDirection: "column",
                }}
                onClick={() => {
                  setPlaylistId(playlist.id);
                  navigate("/playlist");
                }}
              >
                <Avatar src={playlist.coverImgUrl} />
                <Typography 
                  sx={{
                    lineClamp: "2",
                    wordBreak: "break-all",
                    textOverflow: "ellipsis",
                    overflow: "hidden",
                    fontSize: 10,
                    width: "15ch",
                    marginTop: "1ch",
                    color: "black"
                  }}
                >
                  {playlist.name}
                </Typography>
              </Button>
            </Grid>
          ))}
        </Grid>
      </Paper>
    </Stack>
  );
};